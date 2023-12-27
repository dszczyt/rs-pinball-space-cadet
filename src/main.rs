pub mod messages;
pub mod partman;
pub mod pinball_table;
pub mod table;

use anyhow::Context;
use bytes::Buf;
use num::FromPrimitive;
use partman::{
    bitmap_8bpp::Bitmap8Bpp, colors::Colors, dat, entry::EntryType, table_size::TableSize,
};
use sdl2::{
    event::Event,
    messagebox::{show_simple_message_box, MessageBoxFlag},
    rect::Rect,
};
use sdl2::{keyboard::Keycode, render::TextureCreator, video::WindowContext};
use sdl2::{pixels::Color, render::Texture};
use std::{borrow::BorrowMut, cell::RefCell, convert::Into, ffi::CString, rc::Rc, sync::Arc};
use std::{io::Cursor, time::Duration};
use tokio::sync::{broadcast, Mutex};

use crate::partman::{
    entry::{EntryPalette, EntryShortArray},
    table_objects::ObjectType,
};
use crate::{messages::Message, messages::MessageHandler, partman::dat::Dat};

pub trait Redraw {
    fn redraw(&mut self) -> Result<(), String>;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dat_file = include_bytes!("data/PINBALL.DAT");
    // let dat_file = include_bytes!("data/FONT.DAT");

    let dat_contents = Dat::from_reader(&mut Cursor::new(dat_file).reader())?;

    let title_group = dat_contents
        .groups
        .get(0)
        .context("groups not found")?
        .clone();
    let title_entry = title_group
        .get_entry(EntryType::String)
        .context("no title found")?;
    let tmp: Vec<u8> = title_entry.data.clone().context("???")?.into();
    let title = CString::from_vec_with_nul(tmp)?;
    dbg!(&title);

    let table_size_group = dat_contents
        .get_group_by_name("table_size".to_owned())
        .context("group table size not found")?
        .clone();
    let table_size: TableSize = table_size_group.clone().into();

    // let pbmsg_ft = dat_contents
    //     .get_group_by_name("pbmsg_ft".to_owned())
    //     .unwrap();
    // dbg!(&pbmsg_ft);

    let bg = dat_contents
        .get_group_by_name("background".to_owned())
        .context("group background not found")?;

    let bg_bitmap: Bitmap8Bpp = bg.try_into().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = match video_subsystem
        .window(
            "3D Pinball for Windows - Space Cadet",
            table_size.width * 2,
            table_size.height * 2,
        )
        // .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(e) => {
            show_simple_message_box(
                MessageBoxFlag::ERROR,
                "Could not create window",
                &e.to_string(),
                None,
            )?;
            return Ok(());
        }
    };

    let mut canvas = window.into_canvas().build()?;

    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    let palette: EntryPalette = bg.into();

    let colors: Colors = palette.into();

    let bg_texture = bg_bitmap.texture(&colors, &texture_creator);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let table_objects_group = dat_contents
        .get_group_by_name("table_objects".to_string())
        .context("group not found in table_objects")?;

    let table_objects: EntryShortArray = table_objects_group.into();
    let table_objects: Vec<&[i16]> = table_objects.short_array[1..].chunks(2).collect(); // the first integer is unknown (https://github.com/k4zmu2a/SpaceCadetPinball/blob/master/Doc/.dat%20file%20format.txt#L82)

    canvas.set_scale(2.0, 2.0).unwrap();

    // dbg!(&table_objects);

    // let (tx, mut rx) = broadcast::channel::<Message>(1);

    let mut objects: Vec<Box<dyn Redraw>> = vec![];

    table_objects
        .iter()
        .try_for_each(|pair| -> anyhow::Result<()> {
            let (object_type, group_index) = (pair[0], pair[1]);
            let object_type = FromPrimitive::from_i16(object_type);
            let group = dat_contents
                .groups
                .get(group_index as usize)
                .context("nothing found at index")?;

            match object_type {
                Some(ObjectType::LeftFlipper) => {
                    dbg!(&object_type, &group.name());
                    objects.push(Box::new(table::LeftFlipper::new(group)));
                }
                _ => {}
            }
            Ok(())
        })?;

    'running: loop {
        i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.set_draw_color(Color::RGB(0, 64, 255));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    println!("sending event left pressed");
                    // tx.send(Message::LeftFlipperInputPressed).unwrap();
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    println!("sending event left released");
                    // tx.send(Message::LeftFlipperInputReleased).unwrap();
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    println!("right released");
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // objects.iter_mut().for_each(|object| {
        //     object.as_mut().redraw().unwrap();
        //     let texture = object.bitmap_8bpp.texture(&colors, &texture_creator);
        // });

        table_objects
            .iter()
            // .filter(|pair| {
            //     let (val1, _val2) = (pair[0], pair[1]);
            //     val1 == (ObjectType::Bumper as i16) // 1005 // bumper?
            // })
            .filter(|pair| {
                let object_type = pair[0];
                let object_type: Option<ObjectType> = FromPrimitive::from_i16(object_type);
                matches!(
                    object_type,
                    Some(ObjectType::Bumper) | Some(ObjectType::LeftFlipper)
                )
            })
            .try_for_each(|pair| -> anyhow::Result<()> {
                let (object_type, group_index) = (pair[0], pair[1]);
                let object_type: Option<ObjectType> = FromPrimitive::from_i16(object_type);
                let group = dat_contents
                    .groups
                    .get(group_index as usize)
                    .context("group not found")?;

                // dbg!(&object_type, &group.name());
                // dbg!(&val1, &val2, &group, &group.name());
                let bitmap: Bitmap8Bpp = group.try_into().unwrap();

                let texture = bitmap.texture(&colors, &texture_creator);
                canvas
                    .copy(
                        &texture,
                        None,
                        Some(Rect::new(
                            bitmap.position_x as i32,
                            bitmap.position_y as i32,
                            bitmap.width,
                            bitmap.height,
                        )),
                    )
                    .unwrap();

                Ok(())
            })?;

        // canvas
        //     .copy(
        //         &bg_texture,
        //         None,
        //         Some(Rect::new(
        //             bg_bitmap.position_x as i32,
        //             bg_bitmap.position_y as i32,
        //             bg_bitmap.width as u32,
        //             bg_bitmap.height as u32,
        //         )),
        //     )
        //     .unwrap();

        canvas.present();
        // interval.tick().await;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
