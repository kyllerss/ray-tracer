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

use log::{error, info};
use std::env;
use std::io::Error;
use std::process::exit;

#[rustfmt::skip::macros(vec)]
fn main() -> Result<(), Error> {
    env_logger::init();
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
        error!("usage: [RUST_LOG=trace] ./ray-tracer --ch <chapter number to execute>");
        exit(1);
    }

    match chapter {
        1 => {
            info!("Chapter 1...");
            ch1::run();
            Ok(())
        }
        2 => {
            info!("Chapter 2...");
            ch2::run()
        }
        3 => {
            info!("Nothing to do for chapter 3!");
            Ok(())
        }
        4 => {
            info!("Chapter 4...");
            ch4::run()
        }
        5 => {
            info!("Chapter 5...");
            ch5::run()
        }
        6 => {
            info!("Chapter 6...");
            ch6::run()
        }
        7 => {
            info!("Chapter 7..");
            ch7::run()
        }
        9 => {
            info!("Chapter 9..");
            ch9::run()
        }
        10 => {
            info!("Chapter 10..");
            ch10::run()
        }
        11 => {
            info!("Chapter 11..");
            ch11::run()
        }
        _ => {
            panic!("Unsupported chapter {}!", chapter);
        }
    }
}
