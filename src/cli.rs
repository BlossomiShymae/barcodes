use clap::{Parser, Subcommand};

use crate::types::HexadecimalColor;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Flags,
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
