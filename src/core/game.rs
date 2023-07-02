use super::{board::Board, utils::*, PieceType};

pub struct Game {
    pub board: Board,
    /// if white's turn, true, else false
    pub turn: bool,
    /// the amount of points white has
    pub points: u32,
    /// Fullmove number: The number of the full moves. It starts at 1 and is incremented after Black's move.
    pub fullmove: u32,
    /// Halfmove clock, the amount of moves since the last capture or pawn move used for the 50 move rule
    pub halfmove_clock: u32,
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

        Game {
            board,
            turn,
            points: 0,
            halfmove_clock,
            fullmove,
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
