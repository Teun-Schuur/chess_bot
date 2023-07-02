pub mod core;
pub mod renderer;

use crate::core::{board::Board, game::Game, utils::*, PieceType};
use crate::renderer::{input_handler::InputHandler, renderer::Renderer, textures::Textures};
use std::cmp::min;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess - Dyscalcuchess".to_owned(),
        window_width: 1920,
        window_height: 1080,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new("RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr w KQkq - 0 1");
    // let mut game = Game::new("RNBQKBNR/P1P2P1P/3KP3/8/1Ppppp2/7P/pp4p1/rnbqkbnr w KQkq c3 0 1");
    let mut renderer = Renderer::new().await;

    loop {
        if let Some(r#move) = renderer.user_inputs(&game) {
            game.board.move_piece(&r#move);
        }
        renderer.render(&game);
        next_frame().await
    }
}
