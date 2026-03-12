mod debug;

use gb_core::utils::{DISPLAY_BUFFER, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Read;
use gb_core::cpu::Cpu;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::process::exit;
use gb_core::io::Button;
use std::io::prelude::*;

const SCALE: u32 = 3;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

use crate::debug::Debugger;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let mut gbd = Debugger::new();
    let mut gb = Cpu::new();
    let filename = &args[1];
    let rom = load_rom(filename);
    gb.load_rom(&rom);
    load_battery_save(&mut gb, filename);
    let title = gb.get_title();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(title, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();
    'gameloop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
                Event::KeyDown {keycode: Some(Keycode::Space), ..} => {
                    gbd.set_debugging(true);
                },
                Event::KeyDown{keycode: Some(keycode), ..} => {
                    if let Some(button) = key2btn(keycode) {
                        gb.press_button(button, true);
                    }
                },
                Event::KeyUp{keycode: Some(keycode), ..} => {
                    if let Some(button) = key2btn(keycode) {
                        gb.press_button(button, false);
                    }
                },
                _ => {}
            }
        }
        tick_until_draw(&mut gb, &mut gbd, filename);
        let frame = gb.render();
        draw_screen(&frame, &mut canvas);
    }
}

fn draw_screen(data: &[u8], canvas: &mut Canvas<Window>) {
    for i in (0..DISPLAY_BUFFER).step_by(4) {
        canvas.set_draw_color(Color::RGB(data[i], data[i+1], data[i+2]));
        let pixel = i/4;
        let x = (pixel % SCREEN_WIDTH) as u32;
        let y = (pixel / SCREEN_WIDTH) as u32;

        let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
        canvas.fill_rect(rect).unwrap();
    }
    canvas.present();
}

fn load_rom(path: &str) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut f = File::open(path).expect("Error opening rom file");

    f.read_to_end(&mut buffer).expect("Error reading rom file");
    buffer
}

fn tick_until_draw(gb: &mut Cpu, gbd: &mut Debugger, gamename: &str) {
    loop {
        let render = gb.tick();

        gbd.check_breakpoints(gb.get_pc());
        if gbd.is_debugging() {
            gbd.print_info();
            let quit = gbd.debugloop(gb);
            if quit {
                exit(0);
            }
        }

        if render {
            break;
        }
    }

    if gb.is_battery_dirty() {
        write_battery_save(gb, &gamename);
    }
}

fn write_battery_save(gb: &mut Cpu, gamename: &str) {
    if gb.has_battery() {
        let battery_data = gb.get_battery_data();
        let mut filename = gamename.to_owned();
        filename.push_str(".sav");

        let mut file = OpenOptions::new().write(true).create(true).open(filename).expect("Error openeing save file");
        file.write(battery_data).unwrap();
        gb.clean_battery();
    }
}

fn load_battery_save(gb: &mut Cpu, gamename: &str) {
    if gb.has_battery() {
        let mut battery_data: Vec<u8> = Vec::new();
        let mut filename = gamename.to_owned();
        filename.push_str(".sav");

        let f = OpenOptions::new().read(true).open(filename);
        if f.is_ok() {
            f.unwrap().read_to_end(&mut battery_data).expect("Error reading save file");
            gb.set_battery_data(&battery_data);
        }
    }
}

fn key2btn(key: Keycode) -> Option<Button> {
    match key {
        Keycode::Down =>        { Some(Button::Down)   },
        Keycode::Up =>          { Some(Button::Up)     },
        Keycode::Left =>        { Some(Button::Left)   },
        Keycode::Right =>       { Some(Button::Right)  },
        Keycode::Return =>      { Some(Button::Start)  },
        Keycode::Backspace =>   { Some(Button::Select) },
        Keycode::X =>           { Some(Button::A)      },
        Keycode::Z =>           { Some(Button::B)      },
        _ =>                    { None                 },
    }
}
