use std::{
    collections::HashMap,
    env, fs,
    time::{Duration, Instant},
};

use anyhow::Result;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
};

mod cli;
mod cpu;
mod gpu;
mod util;

const SCALING_FACTOR: usize = 12;
const WINDOW_WIDTH: usize = gpu::SCREEN_WIDTH * SCALING_FACTOR;
const WINDOW_HEIGHT: usize = gpu::SCREEN_HEIGHT * SCALING_FACTOR;

const RGB332_BLACK: u8 = u8::MIN;
const RGB332_WHITE: u8 = u8::MAX;

const TARGET_SPEED: usize = 60;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let program = fs::read(path).unwrap();

    let mut keymap = HashMap::new();
    keymap.insert(Keycode::Num1, 0x1u8);
    keymap.insert(Keycode::Num2, 0x2u8);
    keymap.insert(Keycode::Num3, 0x3u8);
    keymap.insert(Keycode::Num4, 0xCu8);
    keymap.insert(Keycode::Q, 0x4u8);
    keymap.insert(Keycode::W, 0x5u8);
    keymap.insert(Keycode::E, 0x6u8);
    keymap.insert(Keycode::R, 0xDu8);
    keymap.insert(Keycode::A, 0x7u8);
    keymap.insert(Keycode::S, 0x8u8);
    keymap.insert(Keycode::D, 0x9u8);
    keymap.insert(Keycode::F, 0xEu8);
    keymap.insert(Keycode::Z, 0xAu8);
    keymap.insert(Keycode::X, 0x0u8);
    keymap.insert(Keycode::C, 0xBu8);
    keymap.insert(Keycode::V, 0xFu8);

    let mut gpu = gpu::Gpu::new();
    let mut cpu = cpu::Cpu::new(&mut gpu);

    cpu.load(&program);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("reimu", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let clear_color = Color::RGB(0, 0, 0);

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(clear_color);
    canvas.clear();
    canvas.present();

    let tex_creator = canvas.texture_creator();
    let mut texture = tex_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB332,
            gpu::SCREEN_WIDTH as u32,
            gpu::SCREEN_HEIGHT as u32,
        )
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let target_elapsed = Duration::from_nanos(util::ns_per_frame(TARGET_SPEED));
    let mut total_elapsed = Duration::ZERO;

    'main: loop {
        let now = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::F1),
                    ..
                } => {
                    cpu.dump();
                }
                Event::KeyDown {
                    keycode: Some(kc), ..
                } => {
                    if let Some(key) = keymap.get(&kc) {
                        cpu.set_key(*key, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(kc), ..
                } => {
                    if let Some(key) = keymap.get(&kc) {
                        cpu.set_key(*key, false);
                    }
                }
                _ => {}
            }
        }

        cpu.step();

        let screen = cpu.screen();
        let grid: Vec<u8> = screen
            .iter()
            .map(|p| if *p { RGB332_WHITE } else { RGB332_BLACK })
            .collect();

        texture.update(None, &grid, gpu::SCREEN_WIDTH).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        total_elapsed += now.elapsed();
        if total_elapsed >= target_elapsed {
            total_elapsed -= target_elapsed;
            cpu.tick_timers();
        }
    }

    Ok(())
}
