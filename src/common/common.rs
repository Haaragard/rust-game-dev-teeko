use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Message {
    DropPiece(usize, usize),
    DoMove,
    UndoMove,
}

pub struct System {
    pub message_queue: Rc<RefCell<Vec<Message>>>,
    observers: Vec<Rc<RefCell<Vec<Message>>>>,
}

impl System {
    pub fn new() -> Self {
        Self {
            message_queue: Rc::new(RefCell::new(Vec::new())),
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Rc<RefCell<Vec<Message>>>) {
        self.observers.push(observer);
    }

    pub fn publish(&mut self, message: Message) {
        for observer in self.observers.iter() {
            observer
                .borrow_mut()
                .push(message);
        }
    }
}
