use std::io::{self, Write};

use clap::Parser;
use cli::Cli;
use colorful::{Colorful, RGB};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod cli;
pub mod types;

pub const TYPE_SORTS: [&str; 4] = ["█", "▊", "▌", "▎"];

fn main() {
    let cli = Cli::parse();
    let colors = cli.command.colors();

    let size = termsize::get().unwrap();
    let length = colors.len() as u16;
    let chunk = size.cols / length;

    let mut buffer = String::from("");

    for i in 0..length {
        for _ in 0..chunk {
            let mut color = *colors.choose(&mut thread_rng()).unwrap();
            if !cli.random {
                color = *colors.get(i as usize).unwrap();
            }

            if cli.solid {
                buffer.push_str(format!("{}", TYPE_SORTS[0].color(RGB::from(color))).as_str());
                continue;
            }

            match cli.character {
                None => buffer.push_str(
                    format!(
                        "{}",
                        TYPE_SORTS
                            .choose(&mut thread_rng())
                            .unwrap()
                            .color(RGB::from(color))
                    )
                    .as_str(),
                ),
                Some(ref char) => {
                    buffer.push_str(format!("{}", char.clone().color(RGB::from(color))).as_str())
                }
            }
        }
    }

    for _ in 0..cli.length {
        print!("{buffer}");
    }
    let _ = io::stdout().flush();
}
