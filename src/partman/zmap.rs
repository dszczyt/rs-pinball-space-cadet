use bytes::Buf;
use derivative::Derivative;

#[derive(Derivative, Default)]
#[derivative(Debug)]
pub struct Zmap {
    pub width: i16,
    pub height: i16,
    pub pitch: i16, // (half of real value)
    _unknown_1: i32,
    _unknown_2: i16,
    _unknown_3: i16,
    #[derivative(Debug = "ignore")]
    pub data: bytes::Bytes,
}

impl From<bytes::Bytes> for Zmap {
    fn from(bytes: bytes::Bytes) -> Self {
        Self {
            width: bytes.slice(0..2).get_i16_le(),
            height: bytes.slice(2..4).get_i16_le(),
            pitch: bytes.slice(4..6).get_i16_le(),
            _unknown_1: bytes.slice(6..10).get_i32_le(),
            _unknown_2: bytes.slice(10..12).get_i16_le(),
            _unknown_3: bytes.slice(12..14).get_i16_le(),
            data: bytes.slice(14..),
        }
    }
}

impl Zmap {
    pub fn flip_zmap_horizontally(&self) -> bytes::Bytes {
        dbg!(&self);
        self.data
            .rchunks((self.width) as usize)
            .flatten()
            .copied()
            .collect()
    }
}
