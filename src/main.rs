/*  main.rs
 *  Initlizes SDL and chip8, provides core "game loop" */

extern crate sdl2;
extern crate rand;

use std::path::Path;

use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};

mod chip8;

fn main() {
    // Instantiate SDL2
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    // Create a window
    let window = video_ctx
        .window("", 64, 32)
        .position_centered()
        .build()
        .unwrap();

    // Create a rendering context
    let mut canvas = window.into_canvas().target_texture().present_vsync().build().unwrap();

    // Set the drawing color to black
    let _ = canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

    // Clear the buffer, using black
    let _ = canvas.clear();


    let _ = canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 255));

    for y in 0..32 {
        for x in 0..64 {
            let _ = canvas.fill_rect(Rect::new(x, y, 1, 1));
        }
    }

    // Swap out buffer for the present buffer, displaying it.
    let _ = canvas.present();

    let mut events = ctx.event_pump().unwrap();



    //Initialize Chip8 system
    let mut core =  chip8::Chip8{..Default::default()};

    //Load ROM
    core.load_rom();


    // loop until we receive a QuitEvent
    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                //"game loop" code
                Event::KeyDown{..} => {
                    core.emulate_cycle();

                    //Update display
                    for (y, row) in core.display.iter().enumerate() {
                        for (x, pix) in row.iter().enumerate() {
                            if *pix {
                                let _ = canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                            } else {
                                let _ = canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                            }
                            let _ = canvas.fill_rect(Rect::new(x as i32, y as i32, 1, 1));
                        }
                    }

                    let _ = canvas.present();

                    core.dbg();

                    continue;
                },

                _ => {continue;}
            }
        }
    }
}

