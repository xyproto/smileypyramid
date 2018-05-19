#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const VERSION_STRING: &'static str = "Smiley Pyramid 1.0";
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

fn smiley_line(length: u64) -> String {
    let mut sline: String = "".to_owned();
    if length == 1 {
        return ")".to_owned();
    }
    let mut l = length;
    while l > 1 {
        if l % 9 == 0 {
            sline += ":-):-):-)";
            l -= 9;
        } else if l % 5 == 0 {
            sline += ":-):)";
            l -= 5;
        } else if l % 4 == 0 {
            sline += ":):)";
            l -= 4;
        } else if l % 3 == 0 {
            sline += ":-)";
            l -= 3;
        } else if l % 2 == 0 {
            sline += ":)";
            l -= 2;
        } else if l % 2 != 0 {
            sline += ":-)";
            l -= 3;
        }
    }
    sline
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
