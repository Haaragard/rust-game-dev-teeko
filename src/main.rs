use std::{rc::Rc};

use sdl2::{pixels::Color};

pub mod view;

pub mod model;
use crate::{audio::audio::AudioSystem, common::common::{Message, System}, model::game::GameState};

pub mod common;

pub mod audio;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()
        .expect("Could not initiate the SDL2 context.");

    let video_subsystem = sdl_context
        .video()
        .expect("Could not access the video context.");

    let window_build = video_subsystem
        .window("SDL2 Window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("Could not create a window.");

    let window = window_build;

    let canvas = window
        .into_canvas()
        .build()
        .expect("Could not build a canvas.");

    let texture_creator = canvas.texture_creator();

    let mut board_view = view::board_view::Renderer::new(
        canvas,
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &texture_creator
    );

    let mut system = System::new();
    let mut game_state = GameState::new();
    let mut audio_system = AudioSystem::new();

    game_state.system.add_observer(Rc::clone(&audio_system.system.message_queue));
    system.add_observer(Rc::clone(&game_state.system.message_queue));

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not get the event context pump");

    'running: loop {
        board_view.clear(Color::WHITE);

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running;
                },
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    let row: usize = (5 * y / board_view.screen_area.h) as usize;
                    let col: usize = (5 * x / board_view.screen_area.w) as usize;

                    system.publish(Message::DropPiece(row, col));
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), ..} => {
                    system.publish(Message::UndoMove);
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), ..} => {
                    system.publish(Message::DoMove);
                },
                _ => {},
            }
        }

        game_state.update();
        audio_system.update();

        board_view.render(&game_state.board);
        board_view.canvas.present();
    }

    Ok(())
}
