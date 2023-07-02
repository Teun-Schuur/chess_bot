use std::cmp::min;

use macroquad::{
    color::Color,
    color_u8,
    prelude::{mouse_position, vec2, DARKGRAY, WHITE},
    shapes::{draw_circle, draw_circle_lines},
    text::draw_text,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{clear_background, screen_height, screen_width},
};

use crate::core::{board::Board, game::Game, utils::bitboard_to_coordinates, Move, PieceType};

use super::{input_handler::InputHandler, textures::Textures};

const PIECE_SIZE: f32 = 0.9;
const BOARD_SIZE: f32 = 0.8;

pub struct Renderer {
    textures: Textures,
    input_handler: InputHandler,
}

impl Renderer {
    pub async fn new() -> Self {
        let textures = Textures::new().await;
        let input_handler = InputHandler::new();
        Self {
            textures,
            input_handler,
        }
    }

    pub fn user_inputs(&mut self, game: &Game) -> Option<Move> {
        if let Some(move_made) = self.input_handler.handle_input(&game.board) {
            println!("Game: {}", game.exportPosition());
            println!("Move  : {:?}", move_made);
            return Some(move_made);
        }
        None
    }

    pub fn render(&mut self, game: &Game) {
        let position = &game.board;
        let board_size = min(screen_width() as u32, screen_height() as u32) as f32 * BOARD_SIZE;

        clear_background(DARKGRAY);

        draw_texture_ex(
            self.textures.get_board(),
            screen_width() / 2.0 - board_size / 2.0,
            screen_height() / 2.0 - board_size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(board_size, board_size)),
                ..Default::default()
            },
        );

        for column in 0..8 {
            for row in 0..8 {
                if let Some(piece) = position.get_piece_pos(column, row) {
                    let texture = self.textures.get_texture(piece);

                    if self.input_handler.get_picked_piece_pos() == Some(1 << (column + row * 8)) {
                        draw_texture_ex(
                            texture,
                            mouse_position().0 - board_size / 8.0 * PIECE_SIZE / 2.0,
                            mouse_position().1 - board_size / 8.0 * PIECE_SIZE / 2.0,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(
                                    board_size / 8.0 * PIECE_SIZE,
                                    board_size / 8.0 * PIECE_SIZE,
                                )),
                                ..Default::default()
                            },
                        );
                        continue;
                    } else {
                        draw_texture_ex(
                            texture,
                            screen_width() / 2.0 - board_size / 2.0
                                + board_size / 8.0 * column as f32
                                + board_size / 8.0 * (1.0 - PIECE_SIZE) / 2.0,
                            screen_height() / 2.0 - board_size / 2.0
                                + board_size / 8.0 * row as f32
                                + board_size / 8.0 * (1.0 - PIECE_SIZE) / 2.0,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(
                                    board_size / 8.0 * PIECE_SIZE,
                                    board_size / 8.0 * PIECE_SIZE,
                                )),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }

        // light up allowed moves
        if let Some(piece_pos) = self.input_handler.get_picked_piece_pos() {
            let (column, row) = bitboard_to_coordinates(piece_pos);
            let allowed_moves = game.board.get_allowed_moves(column, row);

            for column in 0..8 {
                for row in 0..8 {
                    if allowed_moves & (1 << (column + row * 8)) != 0 {
                        let board_size =
                            min(screen_width() as u32, screen_height() as u32) as f32 * 0.8;
                        let board_pos = vec2(
                            screen_width() / 2.0 - board_size / 2.0,
                            screen_height() / 2.0 - board_size / 2.0,
                        );
                        let board_pos = vec2(board_pos.x, board_pos.y);

                        if game.board.get_piece_pos(column, row).is_some() {
                            draw_circle_lines(
                                board_pos.x + board_size / 8.0 * (column as f32 + 0.5),
                                board_pos.y + board_size / 8.0 * (row as f32 + 0.5),
                                board_size / 8.0 * 0.45,
                                5.0,
                                color_u8!(0x18, 0x16, 0x12, 0x30),
                            );
                        } else {
                            draw_circle(
                                board_pos.x + board_size / 8.0 * (column as f32 + 0.5),
                                board_pos.y + board_size / 8.0 * (row as f32 + 0.5),
                                board_size / 8.0 * 0.2,
                                color_u8!(0x18, 0x16, 0x12, 0x30),
                            );
                        }
                    }
                }
            }
        }
    }
}
