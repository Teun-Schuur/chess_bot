use crate::core::Move;
use crate::core::{board::Board, PieceType};
use macroquad::input::{
    is_mouse_button_pressed, is_mouse_button_released, mouse_position, MouseButton,
};
use macroquad::math::vec2;
use macroquad::window::{screen_height, screen_width};
use std::cmp::min;

pub struct InputHandler {
    picked_piece: Option<PieceType>,
    picked_piece_pos: Option<u64>,
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            picked_piece: None,
            picked_piece_pos: None,
        }
    }

    pub fn handle_input(&mut self, board: &Board) -> Option<Move> {
        let mut move_made = None;
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let board_size = min(screen_width() as u32, screen_height() as u32) as f32 * 0.8;
            let board_pos = vec2(
                screen_width() / 2.0 - board_size / 2.0,
                screen_height() / 2.0 - board_size / 2.0,
            );
            let board_pos = vec2(board_pos.x, board_pos.y);

            let column = ((x - board_pos.x) / (board_size / 8.0)) as u32;
            let row = ((y - board_pos.y) / (board_size / 8.0)) as u32;
            let pos = 1 << (column + row * 8);

            if let Some(piece) = board.get_piece(pos) {
                self.picked_piece = Some(piece);
                self.picked_piece_pos = Some(pos);
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            let (x, y) = mouse_position();
            let board_size = min(screen_width() as u32, screen_height() as u32) as f32 * 0.8;
            let board_pos = vec2(
                screen_width() / 2.0 - board_size / 2.0,
                screen_height() / 2.0 - board_size / 2.0,
            );
            let board_pos = vec2(board_pos.x, board_pos.y);

            let column_to = ((x - board_pos.x) / (board_size / 8.0)) as u32;
            let row_to = ((y - board_pos.y) / (board_size / 8.0)) as u32;

            let pos = 1 << (column_to + row_to * 8);

            if self.picked_piece.is_some() {
                if let Some(piece_pos) = self.picked_piece_pos {
                    let column_from = piece_pos.trailing_zeros() % 8;
                    let row_from = piece_pos.trailing_zeros() / 8;
                    if board.is_legal_move(piece_pos, pos) {
                        move_made = Some(Move {
                            column_from,
                            row_from,
                            column_to,
                            row_to,
                        });
                    }
                }
            }
            self.picked_piece = None;
            self.picked_piece_pos = None;
        }
        move_made
    }

    pub fn get_picked_piece(&self) -> Option<PieceType> {
        self.picked_piece
    }

    pub fn get_picked_piece_pos(&self) -> Option<u64> {
        self.picked_piece_pos
    }
}
