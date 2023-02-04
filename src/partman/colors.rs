use bytes::{Buf, Bytes};
use sdl2::pixels::Color;

use super::entry::EntryPalette;

pub type Colors = Vec<Color>;

impl From<EntryPalette> for Colors {
    fn from(palette: EntryPalette) -> Self {
        palette
            .data
            .chunks(4)
            .into_iter()
            .map(|chunk| Bytes::copy_from_slice(chunk))
            .map(|bytes| {
                Color {
                    b: bytes.slice(0..1).get_u8().into(),
                    g: bytes.slice(1..2).get_u8().into(),
                    r: bytes.slice(2..3).get_u8().into(),
                    a: 0xFF, // bytes.slice(3..4).get_u8().into(),
                }
            })
            .collect()
    }
}
