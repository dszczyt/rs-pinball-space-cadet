use bytes::{Buf, Bytes};
use sdl2::pixels::Color;

use super::entry::EntryPalette;

pub type Colors = Vec<Color>;

impl From<EntryPalette> for Colors {
    fn from(palette: EntryPalette) -> Self {
        palette
            .data
            .chunks(4)
            .map(Bytes::copy_from_slice)
            .map(|bytes| {
                Color {
                    b: bytes.slice(0..1).get_u8(),
                    g: bytes.slice(1..2).get_u8(),
                    r: bytes.slice(2..3).get_u8(),
                    a: 0xFF, // bytes.slice(3..4).get_u8().into(),
                }
            })
            .collect()
    }
}
