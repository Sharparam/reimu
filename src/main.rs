use std::{
    collections::HashMap,
    env, fs,
    time::{Duration, Instant},
};

use anyhow::Result;
use cpu::Cpu;
use egui_backend::{
    egui::{self, Color32, CtxRef, Image, Ui},
    gl,
};
use egui_backend::{DpiScaling, ShaderVersion};
use egui_sdl2_gl as egui_backend;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    video::{GLProfile, SwapInterval},
};

mod cli;
mod cpu;
mod gpu;
mod util;

const SCALING_FACTOR: u32 = 10;

const RENDER_WIDTH: u32 = gpu::SCREEN_WIDTH as u32 * SCALING_FACTOR;
const RENDER_HEIGHT: u32 = gpu::SCREEN_HEIGHT as u32 * SCALING_FACTOR;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;

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
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_framebuffer_srgb_compatible(true);

    let window = video_subsystem
        .window("reimu", WINDOW_WIDTH, WINDOW_HEIGHT)
        // .resizable()
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    // window.gl_make_current(&gl_context).unwrap();

    window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::Immediate)
        .unwrap();

    let (mut egui_painter, mut egui_state) =
        egui_backend::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Custom(2.0));
    let mut egui_ctx = egui::CtxRef::default();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let screen_initial = [Color32::BLACK; gpu::SCREEN_WIDTH * gpu::SCREEN_HEIGHT];
    let screen_texture_id = egui_painter.new_user_texture(
        (gpu::SCREEN_WIDTH, gpu::SCREEN_HEIGHT),
        &screen_initial,
        false,
    );

    let target_elapsed = Duration::from_nanos(util::ns_per_frame(TARGET_SPEED));
    let mut total_elapsed = Duration::ZERO;

    let mut show_debug = true;
    let mut mem_offset: usize = 0;

    let start_time = Instant::now();

    'main: loop {
        let now = Instant::now();

        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        cpu.step();

        if cpu.redraw {
            cpu.redraw = false;
            let screen = cpu.screen();
            let grid: Vec<Color32> = screen
                .iter()
                .map(|p| if *p { Color32::WHITE } else { Color32::BLACK })
                .collect();

            egui_painter.update_user_texture_data(screen_texture_id, &grid);
        }

        egui::Window::new("Screen")
            .collapsible(false)
            .auto_sized()
            .show(&egui_ctx, |ui| {
                ui.add(Image::new(
                    screen_texture_id,
                    egui::vec2(RENDER_WIDTH as f32, RENDER_HEIGHT as f32),
                ));
            });

        if show_debug {
            egui::Window::new("CPU").show(&egui_ctx, |ui| {
                ui_cpu_regs(ui, &mut cpu);
            });
            ui_memory(&egui_ctx, &mut cpu, mem_offset);
        }

        let (egui_output, egui_paint_cmds) = egui_ctx.end_frame();
        egui_state.process_output(&window, &egui_output);
        let paint_jobs = egui_ctx.tessellate(egui_paint_cmds);
        egui_painter.paint_jobs(None, paint_jobs, &egui_ctx.font_image());

        window.gl_swap_window();

        total_elapsed += now.elapsed();
        if total_elapsed >= target_elapsed {
            total_elapsed -= target_elapsed;
            cpu.tick_timers();
        }

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
                Event::KeyUp {
                    keycode: Some(Keycode::F2),
                    ..
                } => {
                    show_debug = !show_debug;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::PageDown),
                    ..
                } => {
                    if mem_offset < 0xF00 {
                        mem_offset += 0x100;
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::PageUp),
                    ..
                } => {
                    if mem_offset > 0x000 {
                        mem_offset -= 0x100;
                    }
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
                _ => egui_state.process_input(&window, event, &mut egui_painter),
            }
        }
    }

    Ok(())
}

fn ui_cpu_regs(ui: &mut Ui, cpu: &mut Cpu) {
    for row in 0..4 {
        ui.columns(8, |cols| {
            for col in 0..4 {
                let idx = col * 4 + row;
                cols[col * 2].label(format!("V{:X}", idx));
                cols[col * 2 + 1].code(format!("{:02X}", cpu.register(idx)));
            }
        });
    }
    ui.columns(8, |cols| {
        cols[0].label("PC");
        cols[1].code(format!("{:04X}", cpu.pc));
        cols[2].label("I");
        cols[3].code(format!("{:04X}", cpu.address_register));
        cols[4].label("DT");
        cols[5].code(format!("{:02X}", cpu.delay_timer));
        cols[6].label("ST");
        cols[7].code(format!("{:02X}", cpu.sound_timer));
    });
    ui.horizontal(|ui| {
        ui.label("SP");
        ui.code(format!("{:02X}", cpu.sp));
    });
    for val in cpu.stack.iter() {
        ui.code(format!("{:04X}", val));
    }
}

fn ui_memory(egui_ctx: &CtxRef, cpu: &mut Cpu, base_offset: usize) {
    egui::Window::new("Memory")
        .min_width(500.0)
        .show(egui_ctx, |ui| {
            ui.columns(17, |cols| {
                for idx in 0..16 {
                    cols[idx + 1].code(format!("{:02X}", idx));
                }
            });
            for row in 0..16 {
                let offset = base_offset + row * 16;
                ui.columns(17, |cols| {
                    cols[0].code(format!("{:03X}", offset));
                    for idx in 0..16 {
                        let pos = offset + idx;
                        cols[idx + 1].code(format!("{:02X}", cpu.memory[pos]));
                    }
                });
            }
        });
}
