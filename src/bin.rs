#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate smileypyramid;

use docopt::Docopt;
use smileypyramid::smiley_line;

const VERSION_STRING: &'static str = "Smiley Pyramid 1.0.3";
const USAGE: &'static str = "
  Usage:
    smileypyramid <width>
    smileypyramid (-h | --help)
    smileypyramid --version

  Options:
    -h --help     Show this screen.
    --version     Show version.
";

const DEFAULT_WIDTH: u64 = 10;

#[derive(Debug, Deserialize)]
struct Args {
    arg_width: Option<u64>,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(VERSION_STRING.to_owned() + "\n" + USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{}", VERSION_STRING);
        std::process::exit(0);
    }
    let width = args.arg_width.unwrap_or(DEFAULT_WIDTH);
    for i in 1..(width+1) {
        println!("{}", smiley_line(i));
    }
}
