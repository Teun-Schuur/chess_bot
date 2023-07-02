/// src/core/mod.rs
/// base module for the core crate
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    BlackBishop,
    BlackKing,
    BlackKnight,
    BlackPawn,
    BlackQueen,
    BlackRook,
    WhiteBishop,
    WhiteKing,
    WhiteKnight,
    WhitePawn,
    WhiteQueen,
    WhiteRook,
}
impl PieceType {
    pub fn from_char(c: char) -> Option<PieceType> {
        match c {
            'p' => Some(PieceType::BlackPawn),
            'r' => Some(PieceType::BlackRook),
            'n' => Some(PieceType::BlackKnight),
            'b' => Some(PieceType::BlackBishop),
            'q' => Some(PieceType::BlackQueen),
            'k' => Some(PieceType::BlackKing),
            'P' => Some(PieceType::WhitePawn),
            'R' => Some(PieceType::WhiteRook),
            'N' => Some(PieceType::WhiteKnight),
            'B' => Some(PieceType::WhiteBishop),
            'Q' => Some(PieceType::WhiteQueen),
            'K' => Some(PieceType::WhiteKing),
            _ => None,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            PieceType::BlackPawn => 'p',
            PieceType::BlackRook => 'r',
            PieceType::BlackKnight => 'n',
            PieceType::BlackBishop => 'b',
            PieceType::BlackQueen => 'q',
            PieceType::BlackKing => 'k',
            PieceType::WhitePawn => 'P',
            PieceType::WhiteRook => 'R',
            PieceType::WhiteKnight => 'N',
            PieceType::WhiteBishop => 'B',
            PieceType::WhiteQueen => 'Q',
            PieceType::WhiteKing => 'K',
        }
    }
    pub fn color(&self) -> bool {
        matches!(
            self,
            PieceType::WhitePawn
                | PieceType::WhiteRook
                | PieceType::WhiteKnight
                | PieceType::WhiteBishop
                | PieceType::WhiteQueen
                | PieceType::WhiteKing
        )
    }
    pub fn points(&self) -> u32 {
        match self {
            PieceType::BlackPawn | PieceType::WhitePawn => 100,
            PieceType::BlackKnight | PieceType::WhiteKnight => 300,
            PieceType::BlackBishop | PieceType::WhiteBishop => 300,
            PieceType::BlackRook | PieceType::WhiteRook => 500,
            PieceType::BlackQueen | PieceType::WhiteQueen => 900,
            PieceType::BlackKing | PieceType::WhiteKing => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub column_from: u32,
    pub row_from: u32,
    pub column_to: u32,
    pub row_to: u32,
}
impl Move {
    pub fn new(from: (u32, u32), to: (u32, u32)) -> Move {
        Move {
            column_from: from.0,
            row_from: from.1,
            column_to: to.0,
            row_to: to.1,
        }
    }

    /// Converts a algebraic notation string to a Move
    /// ex: "a2a4" -> Move { column_from: 0, row_from: 1, column_to: 0, row_to: 3 }
    /// ex: "c1f4" -> Move { column_from: 2, row_from: 0, column_to: 5, row_to: 3 }
    pub fn from_algebraic_notation(notation: &str) -> Result<Move, String> {
        let mut chars = notation.chars();
        let column_from = match chars.next() {
            Some(c) => {
                let c = c as u32 - 97;
                if c > 7 {
                    return Err(String::from("Invalid algebraic notation"));
                }
                c
            }
            None => return Err(String::from("Invalid algebraic notation")),
        };
        let row_from = match chars.next() {
            Some(c) => {
                let c = c as u32 - 49;
                if c > 7 {
                    return Err(String::from("Invalid algebraic notation"));
                }
                c
            }
            None => return Err(String::from("Invalid algebraic notation")),
        };
        let column_to = match chars.next() {
            Some(c) => {
                let c = c as u32 - 97;
                if c > 7 {
                    return Err(String::from("Invalid algebraic notation"));
                }
                c
            }
            None => return Err(String::from("Invalid algebraic notation")),
        };
        let row_to = match chars.next() {
            Some(c) => {
                let c = c as u32 - 49;
                if c > 7 {
                    return Err(String::from("Invalid algebraic notation"));
                }
                c
            }
            None => return Err(String::from("Invalid algebraic notation")),
        };
        if chars.next().is_some() {
            return Err(String::from("Invalid algebraic notation"));
        }
        Ok(Move {
            column_from,
            row_from,
            column_to,
            row_to,
        })
    }

    pub fn to_algebraic_notation(&self) -> String {
        let column_from = (self.column_from + 97) as u8 as char;
        let column_to = (self.column_to + 97) as u8 as char;
        let row_from = 8 - self.row_from;
        let row_to = 8 - self.row_to;
        format!("{}{}{}{}", column_from, row_from, column_to, row_to)
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_algebraic_notation())
    }
}

use macroquad::logging::info;
pub mod board;
pub mod game;
pub mod utils;
