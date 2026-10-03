use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use image::ImageReader;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect, Size};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, List, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use ratatui_image::picker::Picker;
use ratatui_image::protocol::Protocol;
use ratatui_image::{Image, Resize};

/// A preview is specific to one theme at one pane size.
type PreviewKey = (usize, Size);

struct Request {
    key: PreviewKey,
    path: PathBuf,
}

struct Response {
    key: PreviewKey,
    preview: Result<Protocol, String>,
}

struct App<'a> {
    themes: Vec<(&'a String, &'a PathBuf)>,
    current: Option<usize>,
    list: ListState,
    previews: HashMap<PreviewKey, Result<Protocol, String>>,
    pending: Option<PreviewKey>,
    requests: Sender<Request>,
}

/// Lets the user browse themes with a live preview.
/// Returns the chosen image, or `None` if they quit without choosing.
pub fn pick(themes: &BTreeMap<String, PathBuf>, current: Option<&Path>) -> Result<Option<PathBuf>> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal, themes, current);
    ratatui::restore();
    result
}

fn run(
    terminal: &mut DefaultTerminal,
    themes: &BTreeMap<String, PathBuf>,
    current: Option<&Path>,
) -> Result<Option<PathBuf>> {
    // Must run after ratatui::init (alternate screen) and before reading events
    let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::halfblocks());
    let (responses_tx, responses) = mpsc::channel();

    let themes: Vec<_> = themes.iter().collect();
    let current = themes
        .iter()
        .position(|(_, path)| Some(path.as_path()) == current);
    let mut app = App {
        themes,
        current,
        list: ListState::default().with_selected(Some(current.unwrap_or(0))),
        previews: HashMap::new(),
        pending: None,
        requests: spawn_preview_worker(picker, responses_tx),
    };

    loop {
        terminal.draw(|frame| app.draw(frame))?;
        app.receive_previews(&responses);

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => app.list.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => app.list.select_next(),
            KeyCode::Home | KeyCode::Char('g') => app.list.select_first(),
            KeyCode::End | KeyCode::Char('G') => app.list.select_last(),
            KeyCode::Enter => return Ok(Some(app.selected().1.clone())),
            KeyCode::Esc | KeyCode::Char('q') => return Ok(None),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(None),
            _ => {}
        }
    }
}

/// Decoding and scaling a wallpaper is slow, so it happens off the UI thread.
fn spawn_preview_worker(picker: Picker, responses: Sender<Response>) -> Sender<Request> {
    let (requests_tx, requests) = mpsc::channel::<Request>();
    thread::spawn(move || {
        while let Ok(mut request) = requests.recv() {
            // Skip requests superseded while we were busy (e.g. holding an arrow key)
            while let Ok(newer) = requests.try_recv() {
                request = newer;
            }
            let preview = render_preview(&picker, &request).map_err(|err| format!("{err:#}"));
            let response = Response {
                key: request.key,
                preview,
            };
            if responses.send(response).is_err() {
                break;
            }
        }
    });
    requests_tx
}

fn render_preview(picker: &Picker, request: &Request) -> Result<Protocol> {
    let (_, size) = request.key;
    let font = picker.font_size();
    let image = ImageReader::open(&request.path)?
        .with_guessed_format()?
        .decode()?
        // Shrink to the pane's pixel size first; much cheaper to encode
        .thumbnail(
            u32::from(size.width) * u32::from(font.width),
            u32::from(size.height) * u32::from(font.height),
        );
    Ok(picker.new_protocol(image, size, Resize::Fit(None))?)
}

impl App<'_> {
    fn selected(&self) -> (usize, &PathBuf) {
        let index = self.list.selected().unwrap_or(0).min(self.themes.len() - 1);
        (index, self.themes[index].1)
    }

    fn receive_previews(&mut self, responses: &Receiver<Response>) {
        while let Ok(response) = responses.try_recv() {
            if self.pending == Some(response.key) {
                self.pending = None;
            }
            self.previews.insert(response.key, response.preview);
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [main, help] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());
        let longest_name = self
            .themes
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);
        let list_width = (longest_name as u16 + 6).min(main.width / 3);
        let [list_area, preview_area] =
            Layout::horizontal([Constraint::Length(list_width), Constraint::Fill(1)]).areas(main);

        self.draw_list(frame, list_area);
        self.draw_preview(frame, preview_area);
        frame.render_widget(Line::from(" ↑/↓ move · enter apply · q quit").dim(), help);
    }

    fn draw_list(&mut self, frame: &mut Frame, area: Rect) {
        let items = self.themes.iter().enumerate().map(|(index, (name, _))| {
            let marker = if Some(index) == self.current {
                "* "
            } else {
                "  "
            };
            format!("{marker}{name}")
        });
        let list = List::new(items)
            .block(Block::bordered().title(" Themes "))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED | Modifier::BOLD))
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, area, &mut self.list);
    }

    fn draw_preview(&mut self, frame: &mut Frame, area: Rect) {
        let (index, path) = self.selected();
        let path = path.clone();
        let block = Block::bordered().title(" Preview ");
        let inner = block.inner(area);
        frame.render_widget(block, area);
        if inner.is_empty() {
            return;
        }

        let key = (index, inner.as_size());
        match self.previews.get(&key) {
            Some(Ok(preview)) => {
                frame.render_widget(Image::new(preview), centered(inner, preview.size()));
            }
            Some(Err(err)) => {
                let text = format!("Could not load {}:\n{err}", path.display());
                frame.render_widget(Paragraph::new(text).red(), inner);
            }
            None => {
                if self.pending != Some(key) {
                    self.pending = Some(key);
                    let _ = self.requests.send(Request { key, path });
                }
                frame.render_widget(Paragraph::new("Loading…").dim(), inner);
            }
        }
    }
}

fn centered(area: Rect, size: Size) -> Rect {
    let width = size.width.min(area.width);
    let height = size.height.min(area.height);
    Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    }
}
