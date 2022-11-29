use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "toast", about = "The best place to stack your JAM")]
enum Toast {
    /// incremental (idk)
    Incremental {
        /// Activate debug mode
        // short and long flags (-d, --debug) will be deduced from the field's name
        #[structopt(short, long)]
        debug: bool,

        /// Input file
        #[structopt(parse(from_os_str))]
        input_dir: PathBuf,

        /// Output file, stdout if not present
        #[structopt(parse(from_os_str))]
        output_dir: Option<PathBuf>,
    },
}

fn main() {
    let opt = Toast::from_args();
    println!("{:?}", opt);
}