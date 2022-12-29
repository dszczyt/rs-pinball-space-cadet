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

#[derive(Debug, Default)]
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

#[derive(FromPrimitive, Debug, Default, PartialEq)]
pub enum BitmapType {
    None = 0,
    Raw = 1,
    Dib = 2,
    Spliced = 3,
    #[default]
    Undefined = -1,
}

#[derive(Derivative, Default)]
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

impl From<bytes::Bytes> for Bitmap8Bpp {
    fn from(bytes: bytes::Bytes) -> Self {
        let size = bytes.slice(9..13).get_i32_le();
        Self {
            resolution: bytes.slice(0..1).get_i8().into(),
            width: bytes.slice(1..3).get_i16_le() as u32,
            height: bytes.slice(3..5).get_i16_le() as u32,
            position_x: bytes.slice(5..7).get_i16_le(),
            position_y: bytes.slice(7..9).get_i16_le(),
            size: size,
            flags: FromPrimitive::from_i8(bytes.slice(13..14).get_i8())
                .expect("unexpected bitmap flag"),
            data: bytes.slice(14..(size as usize)),
        }
    }
}

impl From<Group> for Bitmap8Bpp {
    fn from(group: Group) -> Self {
        group
            .get_entry(EntryType::Bitmap8bit)
            .unwrap()
            .data
            .clone()
            .unwrap()
            .into()
    }
}

impl Bitmap8Bpp {
    pub fn texture<'a>(
        &'a self,
        colors: &Vec<Color>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Texture {
        if self.flags == BitmapType::Spliced {
            panic!("can't create texture for spliced bitmap")
        }

        let pixel_format = PixelFormatEnum::RGBA32;

        let mut bg_bitmap_content: Vec<u8> = self.data.clone().into();
        let bg_bitmap_content: &mut [u8] = &mut bg_bitmap_content;

        let mut bg_surface =
            Surface::new(self.width as u32, self.height as u32, pixel_format).unwrap();

        bg_bitmap_content.iter().enumerate().for_each(|(i, pixel)| {
            bg_surface
                .fill_rect(
                    Rect::new(
                        i as i32 % (self.width + 1) as i32,
                        self.height as i32 - i as i32 / (self.width + 1) as i32,
                        1,
                        1,
                    ),
                    colors.get(pixel.clone() as usize).unwrap().clone(),
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
        colors: &Vec<Color>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Texture {
        if self.flags != BitmapType::Spliced {
            panic!("can't create texture for non-spliced bitmap")
        }

        let pixel_format = PixelFormatEnum::RGBA32;

        let mut bg_surface = Surface::new(self.width, self.height, pixel_format).unwrap();

        texture_creator
            .create_texture_from_surface(bg_surface)
            .unwrap()
    }
}
