use crate::core::PieceType;
use macroquad::texture::{load_texture, Texture2D};

// data structure for all the pices textures
pub struct Textures {
    black_pawn: Texture2D,
    black_rook: Texture2D,
    black_knight: Texture2D,
    black_bishop: Texture2D,
    black_queen: Texture2D,
    black_king: Texture2D,
    white_pawn: Texture2D,
    white_rook: Texture2D,
    white_knight: Texture2D,
    white_bishop: Texture2D,
    white_queen: Texture2D,
    white_king: Texture2D,
    board: Texture2D,
}

impl Textures {
    #[rustfmt::skip]
    pub async fn new() -> Textures {
        Textures {
            black_pawn:   load_texture("src/assets/chess_pieces/pawnb.png").await.unwrap(),
            black_rook:   load_texture("src/assets/chess_pieces/rookb.png").await.unwrap(),
            black_knight: load_texture("src/assets/chess_pieces/knightb.png").await.unwrap(),
            black_bishop: load_texture("src/assets/chess_pieces/bishopb.png").await.unwrap(),
            black_queen:  load_texture("src/assets/chess_pieces/queenb.png").await.unwrap(),
            black_king:   load_texture("src/assets/chess_pieces/kingb.png").await.unwrap(),
            white_pawn:   load_texture("src/assets/chess_pieces/pawnw.png").await.unwrap(),
            white_rook:   load_texture("src/assets/chess_pieces/rookw.png").await.unwrap(),
            white_knight: load_texture("src/assets/chess_pieces/knightw.png").await.unwrap(),
            white_bishop: load_texture("src/assets/chess_pieces/bishopw.png").await.unwrap(),
            white_queen:  load_texture("src/assets/chess_pieces/queenw.png").await.unwrap(),
            white_king:   load_texture("src/assets/chess_pieces/kingw.png").await.unwrap(),
            board:        load_texture("src/assets/board.png").await.unwrap(),
        }
    }

    #[rustfmt::skip]
    pub fn get_texture(&self, piece: PieceType) -> Texture2D {
        match piece {
            PieceType::BlackPawn   => self.black_pawn,
            PieceType::BlackRook   => self.black_rook,
            PieceType::BlackKnight => self.black_knight,
            PieceType::BlackBishop => self.black_bishop,
            PieceType::BlackQueen  => self.black_queen,
            PieceType::BlackKing   => self.black_king,
            PieceType::WhitePawn   => self.white_pawn,
            PieceType::WhiteRook   => self.white_rook,
            PieceType::WhiteKnight => self.white_knight,
            PieceType::WhiteBishop => self.white_bishop,
            PieceType::WhiteQueen  => self.white_queen,
            PieceType::WhiteKing   => self.white_king,
        }
    }

    pub fn get_board(&self) -> Texture2D {
        self.board
    }
}
