use colorful::{Colorful, RGB};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::data::{TRANSGENDER_FLAG, TYPE_SORTS};

pub mod data;
pub mod types;

fn main() {
    let size = termsize::get().unwrap();

    let length = TRANSGENDER_FLAG.len() as u16;
    let chunk = size.cols / length;

    for _ in 0..length {
        for _ in 0..chunk {
            let color = *TRANSGENDER_FLAG.choose(&mut thread_rng()).unwrap();
            print!(
                "{}",
                TYPE_SORTS
                    .choose(&mut thread_rng())
                    .unwrap()
                    .color(RGB::from(color))
            )
        }
    }
    println!("");
}
