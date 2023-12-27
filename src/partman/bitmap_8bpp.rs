use anyhow::Context;
use bytes::Buf;
use derivative::Derivative;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Texture, TextureCreator},
    surface::Surface,
    video::WindowContext,
};

use super::{entry::EntryType, group::Group};

#[derive(Debug, Default, Clone)]
pub enum Resolution {
    _640x480,
    _800x600,
    _1024x768,
    #[default]
    All = -1,
}

impl From<i8> for Resolution {
    fn from(value: i8) -> Self {
        match value {
            0 => Resolution::_640x480,
            1 => Resolution::_800x600,
            2 => Resolution::_1024x768,
            -1 => Resolution::All,
            _ => unreachable!(),
        }
    }
}

#[derive(FromPrimitive, Debug, Default, PartialEq, Clone)]
pub enum BitmapType {
    None = 0,
    Raw = 1,
    Dib = 2,
    Spliced = 3,
    #[default]
    Undefined = -1,
}

pub const BITMAP_HEADER_SIZE: usize = 14;

#[derive(Derivative, Default, Clone)]
#[derivative(Debug)]
pub struct Bitmap8Bpp {
    pub resolution: Resolution,
    pub width: u32,
    pub height: u32,
    pub position_x: i16,
    pub position_y: i16,
    pub size: i32,
    pub flags: BitmapType,
    #[derivative(Debug = "ignore")]
    pub data: bytes::Bytes,
}

impl TryFrom<bytes::Bytes> for Bitmap8Bpp {
    type Error = anyhow::Error;

    fn try_from(bytes: bytes::Bytes) -> anyhow::Result<Self> {
        let size = bytes.slice(9..13).get_i32_le();
        Ok(Self {
            resolution: bytes.slice(0..1).get_i8().into(),
            width: bytes.slice(1..3).get_i16_le() as u32,
            height: bytes.slice(3..5).get_i16_le() as u32,
            position_x: bytes.slice(5..7).get_i16_le(),
            position_y: bytes.slice(7..9).get_i16_le(),
            size,
            flags: FromPrimitive::from_i8(bytes.slice(13..14).get_i8())
                .context("unexpected bitmap flag")?,
            data: bytes.slice(14..(size as usize)),
        })
    }
}

/*impl From<&Group> for Bitmap8Bpp {
    fn from(group: &Group) -> Self {
        group
            .get_entry(EntryType::Bitmap8bit)
            .unwrap()
            .data
            .clone()
            .unwrap()
            .into()
    }
}*/

impl TryFrom<&Group> for Bitmap8Bpp {
    type Error = anyhow::Error;

    fn try_from(group: &Group) -> anyhow::Result<Self> {
        group
            .get_entry(EntryType::Bitmap8bit)
            .context("can't get entry for bitmap 8bit")?
            .data
            .clone()
            .context("no data".to_owned())?
            .try_into()
    }
}

impl Bitmap8Bpp {
    pub fn texture<'a>(
        &'a self,
        colors: &[Color],
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Texture {
        if self.flags == BitmapType::Spliced {
            panic!("can't create texture for spliced bitmap")
        }

        let pixel_format = PixelFormatEnum::RGBA32;

        let mut bg_bitmap_content: Vec<u8> = self.data.clone().into();
        let bg_bitmap_content: &mut [u8] = &mut bg_bitmap_content;

        if self.flags == BitmapType::Spliced {
            dbg!(&bg_bitmap_content.len(), self.width * self.height);
        }

        let mut bg_surface = Surface::new(self.width, self.height, pixel_format).unwrap();

        bg_bitmap_content.iter().enumerate().for_each(|(i, pixel)| {
            bg_surface
                .fill_rect(
                    Rect::new(
                        i as i32 % (self.width + 1) as i32,
                        self.height as i32 - i as i32 / (self.width + 1) as i32,
                        1,
                        1,
                    ),
                    *colors.get(*pixel as usize).unwrap(),
                )
                .unwrap()
        });

        texture_creator
            .create_texture_from_surface(bg_surface)
            .unwrap()
    }

    pub fn texture_at<'a>(
        &'a self,
        idx: usize,
        colors: &[Color],
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> anyhow::Result<Texture> {
        if self.flags != BitmapType::Spliced {
            return Err(anyhow::anyhow!(
                "can't create texture for non-spliced bitmap"
            ));
        }

        let pixel_format = PixelFormatEnum::RGBA32;

        let bg_surface = Surface::new(self.width, self.height, pixel_format).unwrap();

        Ok(texture_creator.create_texture_from_surface(bg_surface)?)
    }
}
