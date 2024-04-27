use colorful::{Colorful, RGB};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::codes::{BARCODES, TRANSGENDER_BARCODE};

pub mod codes;
pub mod types;

fn main() {
    let size = termsize::get().unwrap();

    let length = TRANSGENDER_BARCODE.hex_codes.len() as u16;
    let chunk = size.cols / length;

    for _ in 0..length {
        for _ in 0..chunk {
            let rgb = *TRANSGENDER_BARCODE
                .hex_codes
                .choose(&mut thread_rng())
                .unwrap();
            print!(
                "{}",
                BARCODES
                    .choose(&mut thread_rng())
                    .unwrap()
                    .color(number_to_rgb(rgb))
            )
        }
    }
    println!("");
}

fn number_to_rgb(n: i32) -> RGB {
    let v: Vec<u8> = format!("{:x}", n)
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| u8::from_str_radix(&c.iter().collect::<String>(), 16).unwrap())
        .collect();

    RGB::new(v[0], v[1], v[2])
}
