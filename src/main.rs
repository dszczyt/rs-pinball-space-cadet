mod partman;
mod pinball_table;

use bytes::{Buf, Bytes};
use partman::{
    bitmap_8bpp::Bitmap8Bpp, colors::Colors, dat, entry::EntryType, table_size::TableSize,
};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{
    event::Event,
    messagebox::{show_simple_message_box, MessageBoxFlag},
    rect::Rect,
};
use std::{convert::Into, ffi::CString};
use std::{io::Cursor, time::Duration};

use crate::partman::{
    entry::{EntryPalette, EntryShortArray},
    table_objects::ObjectType,
};

fn main() {
    let dat_file = include_bytes!("data/PINBALL.DAT");
    // let dat_file = include_bytes!("data/FONT.DAT");

    let dat_contents = dat::Dat::from_reader(&mut Cursor::new(dat_file).reader()).unwrap();

    let title_group = dat_contents.groups.get(0).unwrap().clone();
    let title_entry = title_group.get_entry(EntryType::String).unwrap();
    let tmp: Vec<u8> = title_entry.data.clone().unwrap().into();
    let title = CString::from_vec_with_nul(tmp).unwrap();
    dbg!(&title);

    let table_size_group = dat_contents
        .get_group_by_name("table_size".to_owned())
        .unwrap()
        .clone();
    let table_size: TableSize = table_size_group.clone().into();

    // let pbmsg_ft = dat_contents
    //     .get_group_by_name("pbmsg_ft".to_owned())
    //     .unwrap();
    // dbg!(&pbmsg_ft);

    let bg = dat_contents
        .get_group_by_name("background".to_owned())
        .unwrap();

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
            )
            .unwrap();
            return;
        }
    };

    let mut canvas = window.into_canvas().build().unwrap();

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
        .unwrap();

    let table_objects: EntryShortArray = table_objects_group.into();
    let table_objects: Vec<&[i16]> = table_objects.short_array[1..].chunks(2).collect(); // the first integer is unknown (https://github.com/k4zmu2a/SpaceCadetPinball/blob/master/Doc/.dat%20file%20format.txt#L82)

    canvas.set_scale(2.0, 2.0).unwrap();

    // dbg!(&table_objects);

    'running: loop {
        i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.set_draw_color(Color::RGB(0, 64, 255));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        table_objects
            .iter()
            // .filter(|pair| {
            //     let (val1, _val2) = (pair[0], pair[1]);
            //     val1 == (ObjectType::Bumper as i16) // 1005 // bumper?
            // })
            .for_each(|pair| {
                let (_val1, val2) = (pair[0], pair[1]);
                let group = dat_contents.groups.get(val2 as usize).unwrap();
                // dbg!(&val1, &val2, &group, &group.name());
                let bitmap: Result<Bitmap8Bpp, _> = group.try_into();

                match bitmap {
                    Ok(bitmap) => {
                        let texture = bitmap.texture(&colors, &texture_creator);
                        canvas
                            .copy(
                                &texture,
                                None,
                                Some(Rect::new(
                                    bitmap.position_x as i32,
                                    bitmap.position_y as i32,
                                    bitmap.width as u32,
                                    bitmap.height as u32,
                                )),
                            )
                            .unwrap();
                    }
                    Err(e) => {}
                }
            });

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
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
