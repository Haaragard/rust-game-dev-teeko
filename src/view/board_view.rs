use sdl2::{image::LoadTexture, pixels::Color, rect::{Point, Rect}, render::{Canvas, Texture, TextureCreator}, video::{Window, WindowContext}};

use crate::model::game::BoardPiece;

pub struct Renderer<'a> {
    pub canvas: Canvas<Window>,
    pub screen_area: Rect,
    pub clear_color: Color,
    pieces: Texture<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(
        canvas: Canvas<Window>,
        screen_area: (u32, u32),
        texture_loader: &'a TextureCreator<WindowContext>
    ) -> Self {
        let _pieces = texture_loader
            .load_texture("img/pieces.png")
            .expect("Could no load texture.");

        Self {
            canvas,
            screen_area: Rect::new(
                0, 0,
                screen_area.0,
                screen_area.1,
            ),
            clear_color: Color::BLACK,
            pieces: _pieces,
        }
    }

    pub fn render(&mut self, board: &[[BoardPiece; 5]; 5]) {
        self.draw_lines();

        self.draw_pieces(&board);
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_lines(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);

        let screen_width = self.screen_area.w;
        let screen_height = self.screen_area.h;

        let cell_width: i32 = screen_width / 5;
        let cell_height: i32 = screen_height / 5;

        for i in 0..5 {
            self.canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                Point::new(screen_width - cell_width / 2, cell_height / 2 + i * cell_height)
            ).expect("Could not draw line.");
            self.canvas.draw_line(
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
                Point::new(cell_width / 2 + i * cell_width, screen_height - cell_height / 2)
            ).expect("Could not draw line.");
        }
    }

    fn draw_pieces(&mut self, board: &[[BoardPiece; 5]; 5]) {
        let width = self.screen_area.w /5;
        let height = self.screen_area.h /5;

        for i in 0i32..5 {
            let row: usize = i as usize;
            
            for j in 0i32..5 {
                let col: usize = j as usize;

                if board[row][col] == BoardPiece::None {
                    continue;
                }

                let image = &self.pieces;
                let mut src_rect = Rect::new(0,0,32,32);

                if board[row][col] == BoardPiece::Red {
                    src_rect.set_x(0);
                } else if board[row][col] == BoardPiece::Black {
                    src_rect.set_x(32);
                }

                let dst_rect = Rect::new(
                    (width / 4 + width * j) + 24,
                    (height / 4 + height * i) + 16,
                    32,
                    32
                );

                self.canvas.set_draw_color(Color::WHITE);
                self.canvas.fill_rect(dst_rect)
                    .expect("Could no write Rect");
                self.canvas.copy(image, src_rect, dst_rect)
                    .expect("Could not copy.");
            }
        }
    }
}
