use colorful::RGB;

#[derive(Clone, Copy)]
pub struct HexadecimalColor(pub i32);

impl From<HexadecimalColor> for RGB {
    fn from(value: HexadecimalColor) -> Self {
        let colors: Vec<u8> = format!("{:06x}", value.0)
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|c| u8::from_str_radix(&c.iter().collect::<String>(), 16).unwrap())
            .collect();

        RGB::new(colors[0], colors[1], colors[2])
    }
}
