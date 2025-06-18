use crate::common::common::{Message, System};

#[derive(Clone, Copy, PartialEq)]
pub enum BoardPiece {
    None,
    Red,
    Black,
}

pub struct GameState  {
    pub board: [[BoardPiece; 5]; 5],
    pub current_player: BoardPiece,
    pub pieces_dropped: [i32; 2],
    pub system: System,
    history: Vec<PieceDropCommand>,
    history_pos: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: [[BoardPiece::None; 5]; 5],
            current_player: BoardPiece::Red,
            pieces_dropped: [0, 0],
            system: System::new(),
            history: Vec::new(),
            history_pos: 0,
        }
    }

    pub fn update(&mut self) {
        let messages = {
            let mut message_queue = self.system.message_queue.borrow_mut();
            let messages = message_queue.clone();

            message_queue.clear();
            messages
        };

        for message in messages {
            match message {
                Message::DropPiece(row, col) => self.handle_click(row, col),
                Message::DoMove => self.redo_action(),
                Message::UndoMove => self.undo_action(),
            }
        }
    }

    fn handle_click(&mut self, row: usize, col: usize) {
        let command = PieceDropCommand {
            row,
            col,
            player: self.current_player,
        };

        if !command.is_valid(&self) {
            return;
        }

        if self.history.len() > 0 {
            let elements_to_clear = self.history.len() - (self.history_pos);
            for _ in 0..elements_to_clear {
                self.history.pop();
            }
        }

        command.perform(self);
        self.history.push(command);
        self.history_pos += 1;

        self.system.publish(Message::DoMove);
    }

    pub fn redo_action(&mut self) {
        if self.history_pos as i32 > (self.history.len() as i32 - 1) {
            return;
        }

        let command = self.history[self.history_pos].copy();
        command.perform(self);

        self.history_pos += 1;

        self.system.publish(Message::DoMove);
    }

    pub fn undo_action(&mut self) {
        if self.history_pos == 0 || self.history.len() == 0 {
            return;
        }

        let command: PieceDropCommand = self.history[self.history_pos - 1].copy();
        command.undo(self);

        self.system.publish(Message::UndoMove);

        if self.history_pos == 0 {
            return;
        }
        self.history_pos -= 1;
    }

    pub fn swap_player_turn(&mut self) {
        self.current_player = match self.current_player {
            BoardPiece::Red => BoardPiece::Black,
            BoardPiece::Black => BoardPiece::Red,
            _ => panic!("Not implemented"),
        };
    }

    fn index_of_piece(&self, piece: BoardPiece) -> Result::<usize, ()> {
        match piece {
            BoardPiece::Red => Ok(0),
            BoardPiece::Black => Ok(1),
            _ => Err(()),
        }
    }
}

pub struct PieceDropCommand {
    pub row: usize,
    pub col: usize,
    pub player: BoardPiece,
}

impl PieceDropCommand {
    pub fn perform(&self, game: &mut GameState) {
        game.pieces_dropped[game.index_of_piece(self.player).unwrap()] += 1;
        game.board[self.row][self.col] = self.player;

        game.swap_player_turn();
    }

    pub fn undo(&self, game: &mut GameState) {
        let player_index = game.index_of_piece(self.player).unwrap();
        if game.pieces_dropped[player_index] == 0 {
            return;
        }

        game.pieces_dropped[player_index] -= 1;
        game.board[self.row][self.col] = BoardPiece::None;

        game.swap_player_turn();
    }

    pub fn is_valid(&self, game: &GameState) -> bool {
        if self.row > 4 || self.col > 4 {
            return false;
        }
        if game.board[self.row][self.col] != BoardPiece::None {
            return false;
        }

        let current_player_piece_index_result = game.index_of_piece(game.current_player);
        if current_player_piece_index_result.is_ok() {
            if game.pieces_dropped[current_player_piece_index_result.unwrap()] >= 4 {
                return false;
            }
        }

        return true;
    }

    pub fn copy(&self) -> Self {
        Self {
            row: self.row,
            col: self.col,
            player: self.player,
        }
    }
}
