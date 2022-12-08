# Rs-Themer

A simple cli programm to manage your pywal themes.

## Compatibility

It obviously requires pywal to be installed.
Other than that:
- It's compatible with [pywalfox](https://github.com/Frewacom/pywalfox), just pass the -f flag when selecting a theme.
- If you use [alacritty](https://github.com/alacritty/alacritty) and want to also change the terminal colors using [this script](https://github.com/egeesin/alacritty-color-export), clone the repo and move script.sh somewhere in the $PATH and rename it "alacritty_theme" and pass the -a flag when selecting a theme.
- If you want to also reload qtile so it uses the updated colors, pass the -q flag when selecting a theme.

## Screenshots

![list showcase](screenshots/list.png)
![random](screenshots/random_change.png)

## USAGE:
```
themer <SUBCOMMAND>
```
```
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       <image-path> add new theme to selections
    help      Prints this message or the help of the given subcommand(s)
    list      list available themes
    remove    [theme name] remove theme from selections
    select    change theme to selected
```

### Add

add new theme to selections 

```
themer add <image-path> [theme-name]

ARGS:
    <image-path>    image path
    <theme-name>    theme name (will default to image_name)
```

### Remove

remove theme from selections
```
themer remove [theme-name]

ARGS:
    <theme-name>    theme name
```

### Select


change to selected theme

```
themer select [FLAGS] [theme-name]

FLAGS:
    -a               reload alacritty
    -h, --help       Prints help information
    -f               reload pywalfox
    -q               reload qtile
    -r               select random theme
    -V, --version    Prints version information

ARGS:
    <theme-name>    theme name
```

### List

list available themes

```
themer list
```