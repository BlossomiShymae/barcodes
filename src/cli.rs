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
    Genderfluid,
    NonBinary,
    Asexual,
    Pansexual,
    Aromantic,
    Agender,
    Genderflux,
    Genderqueer,
    Transfeminine,
    Transmasculine,
    Aroace,
    Ally,
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
            Self::Genderfluid => vec![
                HexadecimalColor(0xff75a2),
                HexadecimalColor(0xf5f5f5),
                HexadecimalColor(0xbd18d6),
                HexadecimalColor(0x2c2c2c),
                HexadecimalColor(0x3b3dbd),
            ],
            Self::NonBinary => vec![
                HexadecimalColor(0xfff415),
                HexadecimalColor(0xffffff),
                HexadecimalColor(0x9b59d0),
                HexadecimalColor(0x2d2d2d),
            ],
            Self::Asexual => vec![
                HexadecimalColor(0x2c2c2c),
                HexadecimalColor(0xa3a3a3),
                HexadecimalColor(0xffffff),
                HexadecimalColor(0x800080),
            ],
            Self::Pansexual => vec![
                HexadecimalColor(0x21b1ff),
                HexadecimalColor(0xffd800),
                HexadecimalColor(0xff218c),
            ],
            Self::Aromantic => vec![
                HexadecimalColor(0x3da542),
                HexadecimalColor(0xa7d379),
                HexadecimalColor(0xffffff),
                HexadecimalColor(0xa9a9a9),
                HexadecimalColor(0x2c2c2c),
            ],
            Self::Agender => vec![
                HexadecimalColor(0x2c2c2c),
                HexadecimalColor(0xb9b9b9),
                HexadecimalColor(0xffffff),
                HexadecimalColor(0xb8f483),
                HexadecimalColor(0xffffff),
                HexadecimalColor(0xb9b9b9),
                HexadecimalColor(0x2c2c2c),
            ],
            Self::Genderflux => vec![
                HexadecimalColor(0xf57694),
                HexadecimalColor(0xf2a3b9),
                HexadecimalColor(0xcfcfcf),
                HexadecimalColor(0x7be1f5),
                HexadecimalColor(0x3ecdfa),
                HexadecimalColor(0xfff48c),
            ],
            Self::Genderqueer => vec![
                HexadecimalColor(0xb57edc),
                HexadecimalColor(0xffffff),
                HexadecimalColor(0x4a8123),
            ],
            Self::Transfeminine => vec![
                HexadecimalColor(0x73deff),
                HexadecimalColor(0xffe0ed),
                HexadecimalColor(0xffb5d5),
                HexadecimalColor(0xff8cbe),
                HexadecimalColor(0xffb5d5),
                HexadecimalColor(0xffe0ed),
                HexadecimalColor(0x73deff),
            ],
            Self::Transmasculine => vec![
                HexadecimalColor(0xff8abd),
                HexadecimalColor(0xcdf5fe),
                HexadecimalColor(0x9aebff),
                HexadecimalColor(0x74dfff),
                HexadecimalColor(0x9aebff),
                HexadecimalColor(0xcdf5fe),
                HexadecimalColor(0xff8abd),
            ],
            Self::Aroace => vec![
                HexadecimalColor(0xe28c00),
                HexadecimalColor(0xeccd00),
                HexadecimalColor(0xffffff),
                HexadecimalColor(0x62aedc),
                HexadecimalColor(0x203856),
            ],
            Self::Ally => vec![
                HexadecimalColor(0x000000),
                HexadecimalColor(0xFFFFFF),
                HexadecimalColor(0xF00000),
                HexadecimalColor(0xFE7E00),
                HexadecimalColor(0xFFFF00),
                HexadecimalColor(0x007A41),
                HexadecimalColor(0x4041FE),
                HexadecimalColor(0xA001BE),
            ],
        }
    }
}
