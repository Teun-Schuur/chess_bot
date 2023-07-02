use std::fmt::Debug;

use super::{board::Board, utils::*, Move, PieceType};

pub struct Game {
    pub board: Board,
    /// if white's turn, true, else false
    pub turn: bool,
    /// the amount of points white has
    pub points: i32,
    /// Fullmove number: The number of the full moves. It starts at 1 and is incremented after Black's move.
    pub fullmove: u32,
    /// Halfmove clock, the amount of moves since the last capture or pawn move used for the 50 move rule
    pub halfmove_clock: u32,
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game")
            .field("turn", &self.turn)
            .field("points", &self.points)
            .field("fullmove", &self.fullmove)
            .field("halfmove_clock", &self.halfmove_clock)
            .finish()
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Board::default(),
            turn: true,
            points: 0,
            fullmove: 1,
            halfmove_clock: 0,
        }
    }
}

impl Game {
    pub fn new(FEN: &str) -> Game {
        // split the FEN for every space
        let FEN: Vec<&str> = FEN.split(' ').collect();
        assert!(FEN[1] == "w" || FEN[1] == "b");
        let turn = FEN[1] == "w";
        // castling rights
        let _castling_rights = FEN[2];
        // en passant square

        let mut en_passant_square = 0;
        if let Ok((column, row)) = translate_single_fen(FEN[3]) {
            en_passant_square = coordinates_to_bitboard(column, row);
        };
        // halfmove clock
        let halfmove_clock = FEN[4].parse::<u32>().unwrap();
        // fullmove number
        let fullmove = FEN[5].parse::<u32>().unwrap();
        let board = Board::new(FEN[0], en_passant_square);
        let points = board.points();

        Game {
            board,
            turn,
            points,
            halfmove_clock,
            fullmove,
        }
    }

    pub fn turn(&self) -> bool {
        self.turn
    }

    pub fn make_move(&mut self, r#move: &Move) {
        println!("{:?}", self);
        if let (piece, Some(piece_captured)) = self.board.move_piece(r#move) {
            self.points += if self.turn {
                piece_captured.points() as i32
            } else {
                -(piece_captured.points() as i32)
            };
            match piece {
                PieceType::WhitePawn | PieceType::BlackPawn => {
                    self.halfmove_clock = 0;
                }
                _ => {
                    self.halfmove_clock += 1;
                }
            }
        }
        self.turn = !self.turn;
        if self.turn {
            self.fullmove += 1;
        }
    }

    pub fn exportPosition(&self) -> String {
        let mut FEN = String::new();
        // board
        FEN += &self.board.exportFEN();
        // turn
        FEN += " ";
        FEN += if self.turn { "w" } else { "b" };
        // castling rights
        FEN += " ";
        FEN += "KQkq";
        // en passant square
        FEN += " ";
        FEN += &self.board.exportEnPassantSquare();
        // halfmove clock
        FEN += " ";
        FEN += &self.halfmove_clock.to_string();
        // fullmove number
        FEN += " ";
        FEN += &self.fullmove.to_string();
        FEN
    }
}
