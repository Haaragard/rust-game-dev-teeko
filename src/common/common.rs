#[derive(Clone, Copy)]
pub enum Message {
    DropPiece(usize, usize),
    DoMove,
    UndoMove,
}

pub struct System {
    pub message_queue: Vec<Message>,
    observers: Vec<Vec<Message>>,
}

impl System {
    pub fn new() -> Self {
        Self {
            message_queue: Vec::new(),
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Vec<Message>) {
        self.observers.push(observer);
    }

    pub fn publish(&mut self, message: Message) {
        for observer in self.observers.iter_mut() {
            observer.push(message);
        }
    }
}
