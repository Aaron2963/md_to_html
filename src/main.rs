use std::env;
use std::fs;
use markdown::to_html_with_options;
use markdown::Options;
use crate::server::serve;
use crate::merge_html::merge;

pub mod server;
pub mod merge_html;

#[derive(Debug)]
struct Params {
    src: String,
    dest: String,
    live: bool,
    print: bool,
}

fn main() {
    let params = handle_args();
    if !validate_args(&params) {
        return;
    }
    let md = fs::read_to_string(&params.src).unwrap();
    let options = Options {
        ..Options::gfm()
    };
    let html = to_html_with_options(&md, &options).unwrap();
    let full_html = merge(&params.src, &html);
    if params.print {
        println!("{}", html);
    }
    if params.dest.len() > 0 {
        fs::write(&params.dest, full_html.clone()).unwrap();
    }
    if params.print {
        println!("{}", html);
    }
    if params.live {
        serve(&full_html);
    }
    println!("Source markdown file: `{}` has been converted to html", params.src);
}

fn handle_args() -> Params {
    let args: Vec<String> = env::args().collect();
    let mut params: Params = Params {
        src: String::from(""),
        dest: String::from(""),
        live: false,
        print: false,
    };
    let mut i = 1;
    while i < args.len() {
        // trim prefix --
        if !args[i].starts_with("--") {
            i += 1;
            continue;
        }
        let arg = args[i].trim_start_matches("--");
        if arg == "src" {
            if i + 1 >= args.len() || args[i + 1].starts_with("--") {
                i += 1;
                continue;
            }
            params.src = args[i + 1].clone().to_string();
        } else if arg == "dest" {
            if i + 1 >= args.len() || args[i + 1].starts_with("--") {
                i += 1;
                continue;
            }
            params.dest = args[i + 1].clone().to_string();
        } else if arg == "live" {
            params.live = true;
        } else if arg == "print" {
            params.print = true;
        }
        i += 1;
    }

    params
}

fn validate_args(params: &Params) -> bool {
    if params.src.len() == 0 {
        println!("src is required");
        return false;
    }
    if params.dest.len() == 0 && !params.live && !params.print {
        println!("dest|live|print is required");
        return false;
    }

    true
}