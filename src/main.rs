use bytes::{Buf, Bytes};
use rs_pinball_space_cadet::partman::{bitmap_8bpp::Bitmap8Bpp, dat, entry::EntryType};
use sdl2::pixels::Color;
use sdl2::{
    event::Event,
    messagebox::{show_simple_message_box, MessageBoxFlag},
    pixels::Palette,
    rect::Rect,
    surface::Surface,
};
use sdl2::{keyboard::Keycode, pixels::PixelFormatEnum};
use std::convert::Into;
use std::{io::Cursor, time::Duration};

fn main() {
    let dat_file = include_bytes!("data/PINBALL.DAT");
    // let dat_file = include_bytes!("data/FONT.DAT");

    let dat_contents = dat::Dat::from_reader(&mut Cursor::new(dat_file).reader()).unwrap();

    let table_size = dat_contents
        .get_group_by_name("table_size".to_owned())
        .unwrap();
    dbg!(&table_size);

    let pbmsg_ft = dat_contents
        .get_group_by_name("pbmsg_ft".to_owned())
        .unwrap();
    dbg!(&pbmsg_ft);

    let bg = dat_contents
        .get_group_by_name("background".to_owned())
        .unwrap();

    let bg_bitmap: Bitmap8Bpp = bg.into();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = match video_subsystem
        .window("3D Pinball for Windows - Space Cadet", 600, 416)
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

    let pixel_format = PixelFormatEnum::RGBA32;

    let mut bg_bitmap_content: Vec<u8> = bg_bitmap.data.0.clone().into();
    let bg_bitmap_content: &mut [u8] = &mut bg_bitmap_content;

    let tmp = bg
        .get_entry(EntryType::Palette)
        .unwrap()
        .data
        .clone()
        .unwrap();

    let colors: Vec<Color> = tmp
        .chunks(4)
        .into_iter()
        // .take(245)
        .map(|bytes| {
            let bytes = Bytes::copy_from_slice(bytes);
            let color = Color {
                r: bytes.slice(0..1).get_u8().into(),
                g: bytes.slice(1..2).get_u8().into(),
                b: bytes.slice(2..3).get_u8().into(),
                a: 0xFF, // bytes.slice(3..4).get_u8().into(),
            };
            color
        })
        .collect();

    let mut bg_surface = Surface::new(
        bg_bitmap.width as u32,
        bg_bitmap.height as u32,
        pixel_format,
    )
    .unwrap();

    bg_bitmap_content.iter().enumerate().for_each(|(i, pixel)| {
        bg_surface
            .fill_rect(
                Rect::new(
                    i as i32 % (bg_bitmap.width + 1) as i32,
                    bg_bitmap.height as i32 - i as i32 / (bg_bitmap.width + 1) as i32,
                    1,
                    1,
                ),
                colors.get(pixel.clone() as usize).unwrap().clone(),
            )
            .unwrap()
    });

    let bg_texture = texture_creator
        .create_texture_from_surface(bg_surface)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.set_draw_color(Color::RGB(0, 64, 255));
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

        canvas
            .copy(
                &bg_texture,
                None,
                Some(Rect::new(
                    bg_bitmap.position_x as i32,
                    bg_bitmap.position_y as i32,
                    bg_bitmap.width as u32,
                    bg_bitmap.height as u32,
                )),
            )
            .unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
