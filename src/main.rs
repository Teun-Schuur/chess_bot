pub mod core;
pub mod renderer;

use crate::core::{board::Board, game::Game, utils::*, PieceType};
use crate::renderer::{input_handler::InputHandler, renderer::Renderer, textures::Textures};
use std::cmp::min;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess - Dyscalcuchess".to_owned(),
        // window_width: 1920,
        // window_height: 1080,
        fullscreen: true,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // let mut game = Game::new("RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr w KQkq - 0 1");
    // let mut game = Game::new("RNBQKBNR/P1P2P1P/3KP3/8/1Ppppp2/7P/pp4p1/rnbqkbnr w KQkq c3 0 1");
    // let mut game =
    // Game::new("rn1qkbnr/1p3pp1/p3p2p/2ppPb1P/3P2P1/2P5/PP1N1P2/R1BQKBNR b KQkq g3 0 8");
    let mut game = Game::new("2R5/5p1k/p2Bp2P/1p2Pp2/1P1P4/P1p5/3bn3/6RK b - - 0 42");
    let mut renderer = Renderer::new().await;

    loop {
        if let Some(r#move) = renderer.user_inputs(&game) {
            game.make_move(&r#move);
        }
        renderer.render(&game);
        next_frame().await
    }
}
