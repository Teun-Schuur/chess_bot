pub fn coordinates_to_bitboard(x: u32, y: u32) -> u64 {
    1 << (x + y * 8)
}

pub fn bitboard_to_coordinates(bitboard: u64) -> (u32, u32) {
    let column = bitboard.trailing_zeros() % 8;
    let row = bitboard.trailing_zeros() / 8;
    (column, row)
}

pub fn translate_single_fen(pos: &str) -> Result<(u32, u32), String> {
    let mut pos = pos.chars();
    // check if the length is a multiple of 2
    if pos.clone().count() % 2 != 0 {
        return Err("Invalid length".to_string());
    }
    let column = pos.next().unwrap();
    let row = pos.next().unwrap();
    let column = match column {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return Err("Invalid column".to_string()),
    };
    let row = match row {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => return Err("Invalid row".to_string()),
    };
    Ok((column, row))
}

/// translate a FEN position to a bitboard
/// example: e3c6
pub fn translateFEN(FEN: &str) -> u64 {
    // itterate overy every pair of chars in FEN
    let mut FEN = FEN.chars();
    let mut bitboard = 0;
    while let Some(pos) = FEN.next() {
        let pos = pos.to_string() + &FEN.next().unwrap().to_string();
        if let Ok((column, row)) = translate_single_fen(&pos) {
            bitboard |= 1 << (column + row * 8);
        } else {
            panic!("Invalid FEN");
        }
    }
    bitboard
}

pub fn translate_coordinates(column: u32, row: u32) -> String {
    let mut FEN = String::new();
    FEN += match column {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => panic!("Invalid column"),
    };
    FEN += match row {
        0 => "1",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        _ => panic!("Invalid row"),
    };
    FEN
}

/// translate a bitboard to a FEN position
/// example: e3c6
pub fn translate_bitboard(bitboard: u64) -> String {
    let mut FEN = String::new();
    let mut bitboard = bitboard;
    while bitboard != 0 {
        let (column, row) = bitboard_to_coordinates(bitboard);
        bitboard &= !(1 << (column + row * 8));
        FEN += &translate_coordinates(column, row);
    }
    FEN
}
