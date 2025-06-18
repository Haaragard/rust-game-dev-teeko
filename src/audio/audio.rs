use std::collections::HashMap;

use sdl2::mixer;

use crate::common::common::{Message, System};

pub struct AudioSystem {
    pub system: System,
    channel: mixer::Channel,
    chunks: HashMap<Message, mixer::Chunk>,
}

impl AudioSystem {
    pub fn new() -> Self {
        let system = System::new();

        mixer::open_audio(
            44180,
            mixer::DEFAULT_FORMAT,
            2,
            4096
        )
            .expect("Could not open audio.");

        let channel = mixer::Channel(0);
        let mut chunks: HashMap<Message, mixer::Chunk> = HashMap::new();

        chunks.insert(
            Message::DoMove,
            mixer::Chunk::from_file("sfx/click.mp3")
                .expect("Could not load click audio.")
        );

        Self { system, channel, chunks }
    }

    pub fn update(&mut self) {
        let messages = {
            let mut message_queue = self.system.message_queue.borrow_mut();
            let messages = message_queue.clone();

            message_queue.clear();
            messages
        };

        for message in messages.iter() {
            if !self.chunks.contains_key(&message) {
                continue;
            }

            let chunk = self.chunks
                .get(&message)
                .expect("Could not get audio chunk.");

            self.channel
                .play(chunk, 0)
                .expect("Could not play audio");
        }
    }
}
