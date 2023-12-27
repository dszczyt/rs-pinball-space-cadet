use bytes::{Buf, Bytes};

use crate::partman::bitmap_8bpp::{Bitmap8Bpp, BITMAP_HEADER_SIZE};

#[derive(Default)]
struct Score {
    pub score: i32,
    pub dirty_flag: bool,
    // pub background_bmp: Bitmap8Bpp,
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: i32,
    pub height: i32,
    pub char_bmp: Box<[Bitmap8Bpp; 10]>,
}

pub enum ScoreFieldName {
    BallCount, // = "ballcount1",
    PlayerNumber,
    Unknown,
}

impl From<&str> for ScoreFieldName {
    fn from(name: &str) -> Self {
        match name {
            "ballcount1" => Self::BallCount,
            "player_number1" => Self::PlayerNumber,
            _ => Self::Unknown,
        }
    }
}

impl TryFrom<Bytes> for Score {
    type Error = anyhow::Error;

    fn try_from(bytes: Bytes) -> anyhow::Result<Self> {
        let mut start = 8;

        let mut char_bmp: [Bitmap8Bpp; 10] = Default::default();
        (0..10).try_for_each(|i| -> anyhow::Result<()> {
            char_bmp[i] = bytes.slice(start..).try_into()?;
            start = start + char_bmp[i].size as usize + BITMAP_HEADER_SIZE;
            Ok(())
        })?;
        Ok(Self {
            score: -9999,
            dirty_flag: true,
            offset_x: bytes.slice(0..2).get_i16_le().into(),
            offset_y: bytes.slice(2..4).get_i16_le().into(),
            width: bytes.slice(4..6).get_i16_le().into(),
            height: bytes.slice(6..8).get_i16_le().into(),
            char_bmp: Box::new(char_bmp), // bytes.slice(8..).into(),
        })
    }
}
