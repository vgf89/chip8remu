/*  main.rs
 *  Initlizes SDL and chip8, provides core "game loop" */

extern crate sdl2;

use std::path::Path;

use self::sdl2::event::{Event,WindowEventId};
use self::sdl2::rect::{Rect, Point};
use self::sdl2::surface::{Surface,SurfaceRef};
use self::sdl2::keyboard::Keycode;

mod chip8;

fn main() {
    // Instantiate SDL2
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    // Create a window
    let window = match video_ctx.window("eg03", 128, 64).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    // Create a rendering context
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    // Set the drawing color to black
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

    // Clear the buffer, using black
    let _ = renderer.clear();


    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 255));

    for y in 0..32 {
        for x in 0..64 {
            let _ = renderer.fill_rect(Rect::new(2*x, 2*y, 2, 2));
        }
    }

    // Swap out buffer for the present buffer, displaying it.
    let _ = renderer.present();

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
                _               => {
                    core.emulate_cycle();
                    continue;
                }
            }
        }
    }
}

