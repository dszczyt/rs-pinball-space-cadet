use std::fmt::Debug;

use bytes::Buf;
use debug_ignore::DebugIgnore;

#[derive(Debug, Default)]
pub struct Zmap {
    pub width: i16,
    pub height: i16,
    pub pitch: i16, // (half of real value)
    _unknown_1: i32,
    _unknown_2: i16,
    _unknown_3: i16,
    pub data: DebugIgnore<bytes::Bytes>,
}

impl From<bytes::Bytes> for Zmap {
    fn from(bytes: bytes::Bytes) -> Self {
        Self {
            width: bytes.slice(0..2).get_i16_le().into(),
            height: bytes.slice(2..4).get_i16_le().into(),
            pitch: bytes.slice(4..6).get_i16_le().into(),
            _unknown_1: bytes.slice(6..10).get_i32_le().into(),
            _unknown_2: bytes.slice(10..12).get_i16_le().into(),
            _unknown_3: bytes.slice(12..14).get_i16_le().into(),
            data: DebugIgnore(bytes.slice(14..)),
        }
    }
}

impl Zmap {
    pub fn flip_zmap_horizontally(&self) -> bytes::Bytes {
        dbg!(&self);
        self.data
            .0
            .rchunks((self.width) as usize)
            .flatten()
            .copied()
            .collect()
    }
}
