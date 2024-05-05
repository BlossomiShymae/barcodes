use clap::{Parser, Subcommand};

use crate::types::HexadecimalColor;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Flags,
    #[arg(
        global = true,
        short = 'l',
        long = "length",
        default_value_t = 1,
        help = "The vertical length of the barcode."
    )]
    pub length: i8,
    #[arg(
        global = true,
        short = 'c',
        long = "character",
        help = "The character to use when drawing the barcode"
    )]
    pub character: Option<String>,
    #[arg(
        global = true,
        short = 'r',
        long = "random",
        help = "The color to randomize when drawing the barcode."
    )]
    pub random: bool,
    #[arg(
        global = true,
        short = 's',
        long = "solid",
        help = "Uses a solid block for drawing the barcode. Overrides the character flag."
    )]
    pub solid: bool,
}

#[derive(Subcommand)]
pub enum Flags {
    Lesbian,
    Gay,
    Bisexual,
    Transgender,
}

impl Flags {
    pub fn colors(self) -> Vec<HexadecimalColor> {
        match self {
            Self::Lesbian => vec![
                HexadecimalColor(0xd52d00),
                HexadecimalColor(0xef7627),
                HexadecimalColor(0xff9a56),
                HexadecimalColor(0xffffff),
                HexadecimalColor(0xff9a56),
                HexadecimalColor(0xef7627),
                HexadecimalColor(0xd52d00),
            ],
            Self::Gay => vec![
                HexadecimalColor(0xe50303),
                HexadecimalColor(0xff8c00),
                HexadecimalColor(0xffed00),
                HexadecimalColor(0x008026),
                HexadecimalColor(0x24408e),
                HexadecimalColor(0x732982),
            ],
            Self::Bisexual => vec![
                HexadecimalColor(0xd60270),
                HexadecimalColor(0x9b4f96),
                HexadecimalColor(0x0038a8),
            ],
            Self::Transgender => vec![
                HexadecimalColor(0x5bcefa),
                HexadecimalColor(0xf5a9b8),
                HexadecimalColor(0xffffff),
                HexadecimalColor(0xf5a9b8),
                HexadecimalColor(0x5bcefa),
            ],
        }
    }
}
