use sdl2::{
    pixels::Color,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use tokio::sync::broadcast::Receiver;

use crate::{
    messages::{Message, MessageHandler},
    partman::{bitmap_8bpp::Bitmap8Bpp, group::Group},
    Redraw,
};

pub struct LeftFlipper {
    pub group: Group,
    pub sprite: usize,
    pub bitmap_8bpp: Bitmap8Bpp,
}

impl LeftFlipper {
    pub fn new(group: &Group) -> LeftFlipper {
        LeftFlipper {
            group: group.clone(),
            sprite: 0,
            bitmap_8bpp: group.try_into().unwrap(),
        }
    }
}

impl MessageHandler for LeftFlipper {
    fn handle(&mut self, message: Message) -> Result<(), String> {
        match message {
            Message::LeftFlipperInputPressed => {
                println!("left pressed");
            }
            Message::LeftFlipperInputReleased => {
                println!("left released");
            }
            _ => {}
        };
        Ok(())
    }
}

impl Redraw for LeftFlipper {
    fn redraw(&mut self) -> Result<(), String> {
        Ok(())
    }
}

/*impl<'a, 'b, 'c: 'a> Redraw<'a, 'b, 'c> for LeftFlipper {
    fn redraw(
        &'a mut self,
        colors: &'b Vec<Color>,
        texture_creator: &'c TextureCreator<WindowContext>,
    ) -> Result<Texture, String> {

        //let texture = self.bitmap_8bpp.texture(colors, texture_creator);
        //Ok(texture)
    }
}*/
