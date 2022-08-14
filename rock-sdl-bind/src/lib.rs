extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::libc::c_char;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::EventPump;
use std::ffi::CStr;
use std::time::Duration;

#[no_mangle]
pub extern "C" fn rock_sdl_init(x: f64, y: f64) -> *mut State {
    Box::into_raw(Box::new(
        rock_sdl_init_internal(x as u32, y as u32).unwrap(),
    ))
}

#[no_mangle]
pub extern "C" fn rock_sdl_print(x: f64) {
    println!("{}", x);
}

pub struct State {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

#[no_mangle]
pub extern "C" fn rock_sdl_render_texture(
    state_ptr: *mut State,
    texture_ptr: *mut Texture,
    x: f64,
    y: f64,
) {
    let mut state = unsafe { Box::from_raw(state_ptr) };
    let texture = unsafe { Box::from_raw(texture_ptr) };

    state
        .canvas
        .copy(&texture, None, Rect::new(x as i32, y as i32, 100, 100))
        .unwrap();
    state.canvas.present();

    Box::leak(state);
    Box::leak(texture);
}

#[no_mangle]
pub extern "C" fn rock_sdl_load_texture<'a>(
    state_ptr: *mut State,
    path_buf: *const c_char,
) -> *mut Texture {
    let path = unsafe { CStr::from_ptr(path_buf).to_str().unwrap() };

    let state = unsafe { Box::from_raw(state_ptr) };
    let texture_creator = state.canvas.texture_creator();
    let texture = texture_creator.load_texture(path).unwrap();
    Box::leak(state);

    Box::into_raw(Box::new(texture))
}

#[no_mangle]
pub extern "C" fn rock_sdl_loop(state_ptr: *mut State) {
    let mut state = unsafe { Box::from_raw(state_ptr) };

    for event in state.event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => panic!(),
            _ => {}
        }
    }

    state.canvas.clear();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

    Box::into_raw(Box::new(state));
}

fn rock_sdl_init_internal(width: u32, height: u32) -> Result<State, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rocksdl", width, height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context.event_pump()?;

    Ok(State { canvas, event_pump })
}
