use sdl2::{mixer, pixels::Color};

pub mod view;

pub mod model;
use crate::model::game::{GameState};

pub mod common;

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

    mixer::open_audio(
        44100,
        mixer::DEFAULT_FORMAT,
        2,
        4096
    )
        .expect("Could not open audio system.");
    let sound_channel = mixer::Channel(0);

    let click_sound = mixer::Chunk::from_file("sfx/click.mp3")
        .expect("Could not load sound.");

    let mut board_view = view::board_view::Renderer::new(
        canvas,
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &texture_creator
    );

    let mut game_board = GameState::new();

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

                    game_board.handle_click(row, col);

                    sound_channel.play(&click_sound, 0)
                        .expect("Could not play click sound.");
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), ..} => {
                    game_board.undo_action();
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), ..} => {
                    game_board.redo_action();

                    sound_channel.play(&click_sound, 0)
                        .expect("Could not play click sound.");
                },
                _ => {},
            }
        }

        board_view.render(&game_board.board);
        board_view.canvas.present();
        // std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
