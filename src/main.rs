mod ch1;
mod ch2;
mod ch4;
pub mod domain;
#[macro_use]
mod macros;
mod ch10;
mod ch11;
mod ch5;
mod ch6;
mod ch7;
mod ch9;
mod tests;
pub mod utils;

use std::env;
use std::io::Error;
use std::process::exit;

#[rustfmt::skip::macros(vec)]
fn main() -> Result<(), Error> {
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
            Ok(())
        }
        2 => {
            println!("Chapter 2...");
            ch2::run()
        }
        3 => {
            println!("Nothing to do for chapter 3!");
            Ok(())
        }
        4 => {
            println!("Chapter 4...");
            ch4::run()
        }
        5 => {
            println!("Chapter 5...");
            ch5::run()
        }
        6 => {
            println!("Chapter 6...");
            ch6::run()
        }
        7 => {
            println!("Chapter 7..");
            ch7::run()
        }
        9 => {
            println!("Chapter 9..");
            ch9::run()
        }
        10 => {
            println!("Chapter 10..");
            ch10::run()
        }
        11 => {
            println!("Chapter 11..");
            ch11::run(3)
        }
        _ => {
            panic!("Unsupported chapter {}!", chapter);
        }
    }
}
