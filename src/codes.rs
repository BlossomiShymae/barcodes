use crate::types::Barcode;

pub const BARCODES: [&str; 3] = ["▊", "▌", "▎"];

pub const TRANSGENDER_BARCODE: Barcode<5> = Barcode {
    hex_codes: [0x5bcefa, 0xf5a9b8, 0xffffff, 0xf5a9b8, 0x5bcefa],
    width: 1,
};
