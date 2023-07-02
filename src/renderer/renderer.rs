use std::cmp::min;

use macroquad::{
    color::Color,
    color_u8,
    prelude::{mouse_position, vec2, DARKGRAY, WHITE},
    shapes::{draw_circle, draw_circle_lines, draw_rectangle},
    text::draw_text,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{clear_background, screen_height, screen_width},
};

use crate::core::{board::Board, game::Game, utils::bitboard_to_coordinates, Move, PieceType};

use super::{input_handler::InputHandler, textures::Textures};

pub const PIECE_SIZE: f32 = 0.9;
pub const BOARD_SIZE: f32 = 0.9;

pub struct Renderer {
    textures: Textures,
    input_handler: InputHandler,
    board_size: f32,
    last_move: Option<Move>,
}

impl Renderer {
    pub async fn new() -> Self {
        let textures = Textures::new().await;
        let input_handler = InputHandler::new();
        let board_size = min(screen_width() as u32, screen_height() as u32) as f32 * BOARD_SIZE;
        Self {
            textures,
            input_handler,
            board_size,
            last_move: None,
        }
    }

    pub fn user_inputs(&mut self, game: &Game) -> Option<Move> {
        if let Some(move_made) = self.input_handler.handle_input(game) {
            // println!("Game: {}", game.exportPosition());
            println!("Move  : {:?}", move_made);
            self.last_move = Some(move_made);
            return Some(move_made);
        }
        None
    }

    pub fn render(&mut self, game: &Game) {
        let position = &game.board;
        self.board_size = min(screen_width() as u32, screen_height() as u32) as f32 * BOARD_SIZE;

        clear_background(DARKGRAY);

        self.render_board();
        self.light_up_previous_move();
        self.render_pieces(position);
        self.light_up_moves(game);
        self.text(game);
    }

    fn render_board(&self) {
        draw_texture_ex(
            self.textures.get_board(),
            screen_width() / 2.0 - self.board_size / 4.0,
            screen_height() / 2.0 - self.board_size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.board_size, self.board_size)),
                ..Default::default()
            },
        );
    }

    fn render_pieces(&self, position: &Board) {
        for column in 0..8 {
            for row in 0..8 {
                if let Some(piece) = position.get_piece_pos(column, row) {
                    let texture = self.textures.get_texture(piece);

                    if self.input_handler.get_picked_piece_pos() == Some(1 << (column + row * 8)) {
                        continue;
                    } else {
                        draw_texture_ex(
                            texture,
                            screen_width() / 2.0 - self.board_size / 4.0
                                + self.board_size / 8.0 * column as f32
                                + self.board_size / 8.0 * (1.0 - PIECE_SIZE) / 2.0,
                            screen_height() / 2.0 - self.board_size / 2.0
                                + self.board_size / 8.0 * row as f32
                                + self.board_size / 8.0 * (1.0 - PIECE_SIZE) / 2.0,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(
                                    self.board_size / 8.0 * PIECE_SIZE,
                                    self.board_size / 8.0 * PIECE_SIZE,
                                )),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
        // draw picked piece
        if let Some(piece_pos) = self.input_handler.get_picked_piece_pos() {
            let (column, row) = bitboard_to_coordinates(piece_pos);
            let piece = position.get_piece_pos(column, row).unwrap();
            let texture = self.textures.get_texture(piece);

            draw_texture_ex(
                texture,
                mouse_position().0 - self.board_size / 8.0 * PIECE_SIZE / 2.0,
                mouse_position().1 - self.board_size / 8.0 * PIECE_SIZE / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        self.board_size / 8.0 * PIECE_SIZE,
                        self.board_size / 8.0 * PIECE_SIZE,
                    )),
                    ..Default::default()
                },
            );
        }
    }

    fn light_up_moves(&self, game: &Game) {
        // light up allowed moves
        if let Some(piece_pos) = self.input_handler.get_picked_piece_pos() {
            let (column, row) = bitboard_to_coordinates(piece_pos);
            let allowed_moves = game.board.get_allowed_moves(column, row, game.turn());

            for column in 0..8 {
                for row in 0..8 {
                    if allowed_moves & (1 << (column + row * 8)) != 0 {
                        let board_pos = vec2(
                            screen_width() / 2.0 - self.board_size / 4.0,
                            screen_height() / 2.0 - self.board_size / 2.0,
                        );
                        let board_pos = vec2(board_pos.x, board_pos.y);

                        if game.board.get_piece_pos(column, row).is_some() {
                            draw_circle_lines(
                                board_pos.x + self.board_size / 8.0 * (column as f32 + 0.5),
                                board_pos.y + self.board_size / 8.0 * (row as f32 + 0.5),
                                self.board_size / 8.0 * 0.45,
                                5.0,
                                color_u8!(0x18, 0x16, 0x12, 0x30),
                            );
                        } else {
                            draw_circle(
                                board_pos.x + self.board_size / 8.0 * (column as f32 + 0.5),
                                board_pos.y + self.board_size / 8.0 * (row as f32 + 0.5),
                                self.board_size / 8.0 * 0.2,
                                color_u8!(0x18, 0x16, 0x12, 0x30),
                            );
                        }
                    }
                }
            }
        }
    }

    fn light_up_previous_move(&self) {
        if let Some(last_move) = self.last_move {
            let board_pos = vec2(
                screen_width() / 2.0 - self.board_size / 4.0,
                screen_height() / 2.0 - self.board_size / 2.0,
            );
            let board_pos = vec2(board_pos.x, board_pos.y);

            draw_rectangle(
                board_pos.x + self.board_size / 8.0 * (last_move.column_from as f32),
                board_pos.y + self.board_size / 8.0 * (last_move.row_from as f32),
                self.board_size / 8.0,
                self.board_size / 8.0,
                // yellow ish
                color_u8!(0xff, 0xff, 0x20, 0x40),
            );

            draw_rectangle(
                board_pos.x + self.board_size / 8.0 * (last_move.column_to as f32),
                board_pos.y + self.board_size / 8.0 * (last_move.row_to as f32),
                self.board_size / 8.0,
                self.board_size / 8.0,
                color_u8!(0xff, 0xff, 0x30, 0x40),
            );
        }
    }

    fn text(&self, game: &Game) {
        let board_pos = vec2(
            screen_width() / 2.0 - self.board_size / 4.0,
            screen_height() / 2.0 - self.board_size / 2.0,
        );
        let board_pos = vec2(board_pos.x, board_pos.y);

        let text = format!("{} to move", if game.turn() { "White" } else { "Black" });

        draw_text(&text, 120., board_pos.y + 100.0, 50.0, WHITE);

        let text = format!(
            "{}{}",
            if game.points >= 0 { "+" } else { "-" },
            game.points / 100
        );
        draw_text(&text, board_pos.x, board_pos.y - 10.0, 30.0, WHITE);
        let text = format!(
            "{}{}",
            if game.points <= 0 { "+" } else { "-" },
            game.points / 100
        );
        draw_text(
            &text,
            board_pos.x,
            board_pos.y + self.board_size + 20.0,
            30.0,
            WHITE,
        );
    }
}
