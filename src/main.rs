mod ch1;
mod ch2;
pub mod domain;
mod tests;
pub mod utils;
use std::env;
use std::process::exit;

fn main() {
    let mut chapter: usize = 0;

    let mut args_itr = env::args();
    let mut error = false;
    if args_itr.len() != 3 {
        error = true;
    } else {
        let _ = args_itr.next();
        let chapter_marker = args_itr.next();

        if chapter_marker.is_some() && chapter_marker.as_ref().unwrap() != "--ch" {
            error = true;
        } else {
            chapter = args_itr.next().as_ref().unwrap().parse().unwrap();
        }
    }

    if error {
        eprintln!("usage: ./ray-tracer --ch <chapter number to execute>");
        exit(1);
    }

    match chapter {
        1 => {
            println!("Chapter 1...");
            ch1::run();
        }
        2 => {
            println!("Chapter 2...");
            ch2::run();
        }
        3 => {
            println!("Chapter 3");
        }
        _ => {
            panic!("Unsupported chapter {}!", chapter);
        }
    }
}
