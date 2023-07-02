// Purpose: Contains the Board struct and its methods.

use super::*;
use crate::core::utils::*;
use crate::core::PieceType;

pub struct Board {
    black_pawn: u64,
    black_rook: u64,
    black_knight: u64,
    black_bishop: u64,
    black_queen: u64,
    black_king: u64,
    white_pawn: u64,
    white_rook: u64,
    white_knight: u64,
    white_bishop: u64,
    white_queen: u64,
    white_king: u64,
    en_passant: u64,
    /// 0b0001 = white can castle kingsid
    /// 0b0010 = white can castle queenside
    /// 0b0100 = black can castle kingside
    /// 0b1000 = black can castle queenside
    castling_rights: u8,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            black_pawn: 0x000000000000FF00,
            black_rook: 0x0000000000000081,
            black_knight: 0x0000000000000042,
            black_bishop: 0x0000000000000024,
            black_queen: 0x0000000000000008,
            black_king: 0x0000000000000010,
            white_pawn: 0x00FF000000000000,
            white_rook: 0x8100000000000000,
            white_knight: 0x4200000000000000,
            white_bishop: 0x2400000000000000,
            white_queen: 0x0800000000000000,
            white_king: 0x1000000000000000,
            en_passant: 0x0000000000000000,
            castling_rights: 0b1111,
        }
    }
}

impl Board {
    /// Creates a new board from a FEN string
    /// See https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    pub fn new(FEN: &str, en_passant: u64) -> Self {
        let mut board = Board::default();
        board.clear();
        board.en_passant = en_passant;
        let mut column = 0;
        let mut row = 0;
        for c in FEN.chars() {
            if c == '/' {
                row += 1;
                column = 0;
            } else if c.is_ascii_digit() {
                column += c.to_digit(10).unwrap();
            } else if c.is_alphabetic() {
                let piece = match c {
                    'p' => PieceType::BlackPawn,
                    'r' => PieceType::BlackRook,
                    'n' => PieceType::BlackKnight,
                    'b' => PieceType::BlackBishop,
                    'q' => PieceType::BlackQueen,
                    'k' => PieceType::BlackKing,
                    'P' => PieceType::WhitePawn,
                    'R' => PieceType::WhiteRook,
                    'N' => PieceType::WhiteKnight,
                    'B' => PieceType::WhiteBishop,
                    'Q' => PieceType::WhiteQueen,
                    'K' => PieceType::WhiteKing,
                    _ => {
                        info!("Unknown character in FEN string: {}", c);
                        PieceType::WhiteKing
                    }
                };
                let pos = 1 << (column + row * 8);
                match piece {
                    PieceType::BlackPawn => board.black_pawn |= pos,
                    PieceType::BlackRook => board.black_rook |= pos,
                    PieceType::BlackKnight => board.black_knight |= pos,
                    PieceType::BlackBishop => board.black_bishop |= pos,
                    PieceType::BlackQueen => board.black_queen |= pos,
                    PieceType::BlackKing => board.black_king |= pos,
                    PieceType::WhitePawn => board.white_pawn |= pos,
                    PieceType::WhiteRook => board.white_rook |= pos,
                    PieceType::WhiteKnight => board.white_knight |= pos,
                    PieceType::WhiteBishop => board.white_bishop |= pos,
                    PieceType::WhiteQueen => board.white_queen |= pos,
                    PieceType::WhiteKing => board.white_king |= pos,
                }
                column += 1;
            } else {
                info!("Unknown character in FEN string: {}", c);
            }
        }
        board
    }

    pub fn exportFEN(&self) -> String {
        let mut FEN = String::new();
        let mut empty = 0;
        for row in (0..8).rev() {
            for column in 0..8 {
                let pos = 1 << (column + row * 8);
                if self.black_pawn & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('p');
                } else if self.black_rook & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('r');
                } else if self.black_knight & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('n');
                } else if self.black_bishop & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('b');
                } else if self.black_queen & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('q');
                } else if self.black_king & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('k');
                } else if self.white_pawn & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('P');
                } else if self.white_rook & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('R');
                } else if self.white_knight & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('N');
                } else if self.white_bishop & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('B');
                } else if self.white_queen & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('Q');
                } else if self.white_king & pos != 0 {
                    if empty > 0 {
                        FEN.push_str(&empty.to_string());
                        empty = 0;
                    }
                    FEN.push('K');
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                FEN.push_str(&empty.to_string());
                empty = 0;
            }
            if row > 0 {
                FEN.push('/');
            }
        }
        FEN
    }

    pub fn exportEnPassantSquare(&self) -> String {
        translate_bitboard(self.en_passant)
    }

    pub fn clear(&mut self) {
        self.black_pawn = 0;
        self.black_rook = 0;
        self.black_knight = 0;
        self.black_bishop = 0;
        self.black_queen = 0;
        self.black_king = 0;
        self.white_pawn = 0;
        self.white_rook = 0;
        self.white_knight = 0;
        self.white_bishop = 0;
        self.white_queen = 0;
        self.white_king = 0;
        self.en_passant = 0;
    }

    pub fn get_piece(&self, pos: u64) -> Option<PieceType> {
        if self.black_pawn & pos != 0 {
            return Some(PieceType::BlackPawn);
        } else if self.black_rook & pos != 0 {
            return Some(PieceType::BlackRook);
        } else if self.black_knight & pos != 0 {
            return Some(PieceType::BlackKnight);
        } else if self.black_bishop & pos != 0 {
            return Some(PieceType::BlackBishop);
        } else if self.black_queen & pos != 0 {
            return Some(PieceType::BlackQueen);
        } else if self.black_king & pos != 0 {
            return Some(PieceType::BlackKing);
        } else if self.white_pawn & pos != 0 {
            return Some(PieceType::WhitePawn);
        } else if self.white_rook & pos != 0 {
            return Some(PieceType::WhiteRook);
        } else if self.white_knight & pos != 0 {
            return Some(PieceType::WhiteKnight);
        } else if self.white_bishop & pos != 0 {
            return Some(PieceType::WhiteBishop);
        } else if self.white_queen & pos != 0 {
            return Some(PieceType::WhiteQueen);
        } else if self.white_king & pos != 0 {
            return Some(PieceType::WhiteKing);
        }
        None
    }

    pub fn get_column_row(&self, pos: u64) -> (u32, u32) {
        let trailing_zeros = pos.trailing_zeros();
        let column = trailing_zeros % 8;
        let row = trailing_zeros / 8;
        (column as u32, row as u32)
    }

    pub fn get_piece_pos(&self, column: u32, row: u32) -> Option<PieceType> {
        let pos = 1 << (column + row * 8);
        self.get_piece(pos)
    }

    pub fn mask(&mut self, mask: u64) {
        self.black_pawn &= !mask;
        self.black_rook &= !mask;
        self.black_knight &= !mask;
        self.black_bishop &= !mask;
        self.black_queen &= !mask;
        self.black_king &= !mask;
        self.white_pawn &= !mask;
        self.white_rook &= !mask;
        self.white_knight &= !mask;
        self.white_bishop &= !mask;
        self.white_queen &= !mask;
        self.white_king &= !mask;
    }

    /// Moves a piece from one position to another.
    /// Returns the piece that has moved and the piece that was captured, if any.
    #[rustfmt::skip]
    pub fn move_piece(
        &mut self,
        r#move: &Move,
    ) -> (PieceType, Option<PieceType>) {
        self.en_passant = 0;
        let from = 1 << (r#move.column_from + r#move.row_from * 8);
        let piece_orgin = self.get_piece(from).unwrap();
        let to = 1 << (r#move.column_to + r#move.row_to * 8);
        let piece_dest = self.get_piece(to);
        self.mask(from | to);
        match piece_orgin {
            PieceType::BlackPawn   => self.black_pawn   |= to,
            PieceType::BlackRook   => self.black_rook   |= to,
            PieceType::BlackKnight => self.black_knight |= to,
            PieceType::BlackBishop => self.black_bishop |= to,
            PieceType::BlackQueen  => self.black_queen  |= to,
            PieceType::BlackKing   => self.black_king   |= to,
            PieceType::WhitePawn   => self.white_pawn   |= to,
            PieceType::WhiteRook   => self.white_rook   |= to,
            PieceType::WhiteKnight => self.white_knight |= to,
            PieceType::WhiteBishop => self.white_bishop |= to,
            PieceType::WhiteQueen  => self.white_queen  |= to,
            PieceType::WhiteKing   => self.white_king   |= to,
        }

        if piece_orgin == PieceType::BlackPawn || piece_orgin == PieceType::WhitePawn {
            if (from << 16) & to != 0 {
                self.en_passant = from << 8;
            } else if (from >> 16) & to != 0 {
                self.en_passant = from >> 8;
            }
        }

        if piece_dest.is_none() {
            if piece_orgin == PieceType::BlackPawn {
                self.white_pawn &= !to >> 8;
                return (piece_orgin, self.get_piece(to - 8));
            } else if piece_orgin == PieceType::WhitePawn {
                self.black_pawn &= !to << 8;
                return (piece_orgin, self.get_piece(to + 8));
            }
        }

        // promotion
        if piece_orgin == PieceType::BlackPawn && r#move.row_to == 7 {
            self.black_pawn &= !to;
            self.black_queen |= to;
        }
        if piece_orgin == PieceType::WhitePawn && r#move.row_to == 0 {
            self.white_pawn &= !to;
            self.white_queen |= to;
        }

        // castling
        if piece_orgin == PieceType::BlackKing {
            if r#move.column_from == 4 && r#move.column_to == 6 {
                self.black_rook &= !(1 << 7);
                self.black_rook |= 1 << 5;
            } else if r#move.column_from == 4 && r#move.column_to == 2 {
                self.black_rook &= !(1 << 0);
                self.black_rook |= 1 << 3;
            }
        }
        if piece_orgin == PieceType::WhiteKing {
            if r#move.column_from == 4 && r#move.column_to == 6 {
                self.white_rook &= !(1 << 63);
                self.white_rook |= 1 << 61;
            } else if r#move.column_from == 4 && r#move.column_to == 2 {
                self.white_rook &= !(1 << 56);
                self.white_rook |= 1 << 59;
            }
        }

        (piece_orgin, piece_dest)
    }

    pub fn get_black_pieces(&self) -> u64 {
        self.black_pawn
            | self.black_rook
            | self.black_knight
            | self.black_bishop
            | self.black_queen
            | self.black_king
    }

    pub fn get_white_pieces(&self) -> u64 {
        self.white_pawn
            | self.white_rook
            | self.white_knight
            | self.white_bishop
            | self.white_queen
            | self.white_king
    }

    pub fn get_all_pieces(&self) -> u64 {
        self.get_black_pieces() | self.get_white_pieces()
    }



    #[rustfmt::skip]
    pub fn get_allowed_moves(&self, column: u32, row: u32, color: bool) -> u64 {
        let piece = self.get_piece_pos(column, row);
        assert!(piece.is_some());
        assert!(piece.unwrap().color() == color);
        let moves = match piece {
            Some(PieceType::WhitePawn)   => self.get_allowed_white_pawn_moves(column, row),
            Some(PieceType::WhiteRook)   => self.get_allowed_white_rook_moves(column, row),
            Some(PieceType::WhiteKnight) => self.get_allowed_white_knight_moves(column, row),
            Some(PieceType::WhiteBishop) => self.get_allowed_white_bishop_moves(column, row),
            Some(PieceType::WhiteQueen)  => self.get_allowed_white_queen_moves(column, row),
            Some(PieceType::WhiteKing)   => self.get_allowed_white_king_moves(column, row),
            Some(PieceType::BlackPawn)   => self.get_allowed_black_pawn_moves(column, row),
            Some(PieceType::BlackRook)   => self.get_allowed_black_rook_moves(column, row),
            Some(PieceType::BlackKnight) => self.get_allowed_black_knight_moves(column, row),
            Some(PieceType::BlackBishop) => self.get_allowed_black_bishop_moves(column, row),
            Some(PieceType::BlackQueen)  => self.get_allowed_black_queen_moves(column, row),
            Some(PieceType::BlackKing)   => self.get_allowed_black_king_moves(column, row),
            None => 0,
        };
        if let Some(piecetype) = piece {
            if color != piece.unwrap().color() {
                return 0;
            }
        }
        moves
    }

    #[rustfmt::skip]
    pub fn is_legal_move(&self, from: u64, to: u64, color: bool) -> bool {
        let (from_column, from_row) = self.get_column_row(from);
        let piece = self.get_piece(from).unwrap();
        if (!piece.color() && color) || (piece.color() && !color) {
            return false;
        }
        match piece {
            PieceType::BlackPawn   => self.get_allowed_black_pawn_moves(from_column, from_row)   & to != 0,
            PieceType::BlackRook   => self.get_allowed_black_rook_moves(from_column, from_row)   & to != 0,
            PieceType::BlackKnight => self.get_allowed_black_knight_moves(from_column, from_row) & to != 0,
            PieceType::BlackBishop => self.get_allowed_black_bishop_moves(from_column, from_row) & to != 0,
            PieceType::BlackQueen  => self.get_allowed_black_queen_moves(from_column, from_row)  & to != 0,
            PieceType::BlackKing   => self.get_allowed_black_king_moves(from_column, from_row)   & to != 0,
            PieceType::WhitePawn   => self.get_allowed_white_pawn_moves(from_column, from_row)   & to != 0,
            PieceType::WhiteRook   => self.get_allowed_white_rook_moves(from_column, from_row)   & to != 0,
            PieceType::WhiteKnight => self.get_allowed_white_knight_moves(from_column, from_row) & to != 0,
            PieceType::WhiteBishop => self.get_allowed_white_bishop_moves(from_column, from_row) & to != 0,
            PieceType::WhiteQueen  => self.get_allowed_white_queen_moves(from_column, from_row)  & to != 0,
            PieceType::WhiteKing   => self.get_allowed_white_king_moves(from_column, from_row)   & to != 0,
        }
    }

    pub fn get_allowed_black_pawn_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves: u64 = 0;
        let pos = 1 << (column + row * 8);
        let all_pieces = self.get_all_pieces();
        // two steps forward
        if row == 1 && ((pos << 16) & !all_pieces != 0) && ((pos << 8) & !all_pieces != 0) {
            moves |= pos << 16;
        }
        // one step forward
        if (pos << 8) & !all_pieces != 0 {
            moves |= pos << 8;
        }
        // capture left
        if column > 0 && (pos << 7) & self.get_white_pieces() != 0 {
            moves |= pos << 7;
        }
        // capture right
        if column < 7 && (pos << 9) & self.get_white_pieces() != 0 {
            moves |= pos << 9;
        }
        // en passant left
        if column > 0 && row == 4 && (pos << 7) & self.en_passant != 0 {
            moves |= pos << 7;
        }
        // en passant right
        if column < 7 && row == 4 && (pos << 9) & self.en_passant != 0 {
            moves |= pos << 9;
        }
        moves
    }

    pub fn get_allowed_white_pawn_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves = 0;
        let pos = 1 << (column + row * 8);
        let all_pieces = self.get_all_pieces();
        // two steps forward
        if row == 6 && ((pos >> 16) & !all_pieces != 0) && ((pos >> 8) & !all_pieces != 0) {
            moves |= pos >> 16;
        }
        // one step forward
        if (pos >> 8) & !all_pieces != 0 {
            moves |= pos >> 8;
        }
        // capture left
        if column > 0 && (pos >> 9) & self.get_black_pieces() != 0 {
            moves |= pos >> 9;
        }
        // capture right
        if column < 7 && (pos >> 7) & self.get_black_pieces() != 0 {
            moves |= pos >> 7;
        }
        // en passant left
        if column > 0 && row == 3 && (pos >> 9) & self.en_passant != 0 {
            moves |= pos >> 9;
        }
        // en passant right
        if column < 7 && row == 3 && (pos >> 7) & self.en_passant != 0 {
            moves |= pos >> 7;
        }
        moves
    }

    pub fn get_allowed_black_rook_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves = 0;
        let pos = 1 << (column + row * 8);
        let mut temp_pos = pos;
        let all_pieces = self.get_all_pieces();
        // up
        for i in 1..8 {
            if row + i > 7 {
                break;
            }
            temp_pos <<= 8;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_white_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // down
        temp_pos = pos;
        for i in 1..8 {
            if row < i {
                break;
            }
            temp_pos >>= 8;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_white_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // left
        temp_pos = pos;
        for i in 1..8 {
            if column < i {
                break;
            }
            temp_pos >>= 1;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_white_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // right
        temp_pos = pos;
        for i in 1..8 {
            if column + i > 7 {
                break;
            }
            temp_pos <<= 1;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_white_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        moves
    }

    pub fn get_allowed_white_rook_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves = 0;
        let pos = 1 << (column + row * 8);
        let mut temp_pos = pos;
        let all_pieces = self.get_all_pieces();
        // up
        for i in 1..8 {
            if row + i > 7 {
                break;
            }
            temp_pos <<= 8;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_black_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // down
        temp_pos = pos;
        for i in 1..8 {
            if row < i {
                break;
            }
            temp_pos >>= 8;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_black_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // left
        temp_pos = pos;
        for i in 1..8 {
            if column < i {
                break;
            }
            temp_pos >>= 1;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_black_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // right
        temp_pos = pos;
        for i in 1..8 {
            if column + i > 7 {
                break;
            }
            temp_pos <<= 1;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_black_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        moves
    }

    pub fn get_allowed_black_knight_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves = 0;
        let pos = 1 << (column + row * 8);
        // up
        if row < 6 {
            if column > 0 {
                moves |= pos << 15;
            }
            if column < 7 {
                moves |= pos << 17;
            }
        }
        // down
        if row > 1 {
            if column > 0 {
                moves |= pos >> 17;
            }
            if column < 7 {
                moves |= pos >> 15;
            }
        }
        // left
        if column > 1 {
            if row > 0 {
                moves |= pos << 6;
            }
            if row < 7 {
                moves |= pos >> 10;
            }
        }
        // right
        if column < 6 {
            if row > 0 {
                moves |= pos << 10;
            }
            if row < 7 {
                moves |= pos >> 6;
            }
        }
        moves & !self.get_black_pieces()
    }

    pub fn get_allowed_white_knight_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves = 0;
        let pos = 1 << (column + row * 8);
        // up
        if row < 6 {
            if column > 0 {
                moves |= pos << 15;
            }
            if column < 7 {
                moves |= pos << 17;
            }
        }
        // down
        if row > 1 {
            if column > 0 {
                moves |= pos >> 17;
            }
            if column < 7 {
                moves |= pos >> 15;
            }
        }
        // left
        if column > 1 {
            if row > 0 {
                moves |= pos << 6;
            }
            if row < 7 {
                moves |= pos >> 10;
            }
        }
        // right
        if column < 6 {
            if row > 0 {
                moves |= pos << 10;
            }
            if row < 7 {
                moves |= pos >> 6;
            }
        }
        moves & !self.get_white_pieces()
    }

    pub fn get_allowed_black_bishop_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves = 0;
        let pos = 1 << (column + row * 8);
        let mut temp_pos = pos;
        let all_pieces = self.get_all_pieces();
        // up left
        for i in 1..8 {
            if row + i > 7 || column < i {
                break;
            }
            temp_pos <<= 7;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_white_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // up right
        temp_pos = pos;
        for i in 1..8 {
            if row + i > 7 || column + i > 7 {
                break;
            }
            temp_pos <<= 9;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_white_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // down left
        temp_pos = pos;
        for i in 1..8 {
            if row < i || column < i {
                break;
            }
            temp_pos >>= 9;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_white_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // down right
        temp_pos = pos;
        for i in 1..8 {
            if row < i || column + i > 7 {
                break;
            }
            temp_pos >>= 7;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_white_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        moves
    }

    pub fn get_allowed_white_bishop_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves = 0;
        let pos = 1 << (column + row * 8);
        let mut temp_pos = pos;
        let all_pieces = self.get_all_pieces();
        // up left
        for i in 1..8 {
            if row + i > 7 || column < i {
                break;
            }
            temp_pos <<= 7;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_black_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // up right
        temp_pos = pos;
        for i in 1..8 {
            if row + i > 7 || column + i > 7 {
                break;
            }
            temp_pos <<= 9;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_black_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // down left
        temp_pos = pos;
        for i in 1..8 {
            if row < i || column < i {
                break;
            }
            temp_pos >>= 9;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_black_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        // down right
        temp_pos = pos;
        for i in 1..8 {
            if row < i || column + i > 7 {
                break;
            }
            temp_pos >>= 7;
            if temp_pos & all_pieces != 0 {
                if temp_pos & self.get_black_pieces() != 0 {
                    moves |= temp_pos;
                }
                break;
            }
            moves |= temp_pos;
        }
        moves
    }

    pub fn get_allowed_black_queen_moves(&self, column: u32, row: u32) -> u64 {
        self.get_allowed_black_bishop_moves(column, row)
            | self.get_allowed_black_rook_moves(column, row)
    }

    pub fn get_allowed_white_queen_moves(&self, column: u32, row: u32) -> u64 {
        self.get_allowed_white_bishop_moves(column, row)
            | self.get_allowed_white_rook_moves(column, row)
    }

    pub fn get_allowed_black_king_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves = 0;
        let pos = 1 << (column + row * 8);
        // up
        if row < 7 {
            moves |= pos << 8;
        }
        // down
        if row > 0 {
            moves |= pos >> 8;
        }
        // left
        if column > 0 {
            moves |= pos >> 1;
        }
        // right
        if column < 7 {
            moves |= pos << 1;
        }
        // up left
        if row < 7 && column > 0 {
            moves |= pos << 7;
        }
        // up right
        if row < 7 && column < 7 {
            moves |= pos << 9;
        }
        // down left
        if row > 0 && column > 0 {
            moves |= pos >> 9;
        }
        // down right
        if row > 0 && column < 7 {
            moves |= pos >> 7;
        }
        // castling right
        if self.castling_rights & 0b0001 != 0
            && self.get_all_pieces() & pos << 1 == 0
            && self.get_all_pieces() & pos << 2 == 0
        {
            moves |= pos << 2;
        }
        // castling left
        if self.castling_rights & 0b0010 != 0
            && self.get_all_pieces() & pos >> 1 == 0
            && self.get_all_pieces() & pos >> 2 == 0
            && self.get_all_pieces() & pos >> 3 == 0
        {
            moves |= pos >> 2;
        }
        moves & !self.get_black_pieces() & !self.get_all_moves(true)
    }

    pub fn get_allowed_white_king_moves(&self, column: u32, row: u32) -> u64 {
        let mut moves = 0;
        let pos = 1 << (column + row * 8);
        // up
        if row < 7 {
            moves |= pos << 8;
        }
        // down
        if row > 0 {
            moves |= pos >> 8;
        }
        // left
        if column > 0 {
            moves |= pos >> 1;
        }
        // right
        if column < 7 {
            moves |= pos << 1;
        }
        // up left
        if row < 7 && column > 0 {
            moves |= pos << 7;
        }
        // up right
        if row < 7 && column < 7 {
            moves |= pos << 9;
        }
        // down left
        if row > 0 && column > 0 {
            moves |= pos >> 9;
        }
        // down right
        if row > 0 && column < 7 {
            moves |= pos >> 7;
        }
        // castling right
        if self.castling_rights & 0b0100 != 0
            && self.get_all_pieces() & pos << 1 == 0
            && self.get_all_pieces() & pos << 2 == 0
        {
            moves |= pos << 2;
        }
        // castling left
        if self.castling_rights & 0b1000 != 0
            && self.get_all_pieces() & pos >> 1 == 0
            && self.get_all_pieces() & pos >> 2 == 0
            && self.get_all_pieces() & pos >> 3 == 0
        {
            moves |= pos >> 2;
        }
        moves & !self.get_white_pieces() & !self.get_all_moves(false)
    }

    /// returns the points that white is ahead by
    pub fn points(&self) -> i32 {
        let mut points = 0;
        for i in 0..64 {
            let pos = 1 << i;
            if let Some(piece) = self.get_piece(pos) {
                points += piece.points() as i32 * (piece.color() as i32 * 2 - 1);
            }
        }
        points
    }

    pub fn get_all_moves(&self, color: bool) -> u64 {
        let mut moves = 0;
        for i in 0..64 {
            let pos = 1 << i;
            if let Some(piece) = self.get_piece(pos) {
                if piece.color() == color {
                    moves |= self.get_allowed_moves(i % 8, i / 8, color);
                }
            }
        }
        moves
    }
}
