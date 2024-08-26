/// a board should be a 32 length array of enums
pub fn make_board(config: BoardConfig ) -> [Piece; 32] {
    match config {
        BoardConfig::Default => {
            return [
                Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn,
                Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn,
                Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn,
                Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, 
                Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, 
                Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, 
                Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, 
                Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, 
                ];
        },
        _ => {
            const PIECE: Piece = Piece::Empty;
            return [PIECE; 32];
        }
    }
    
}

#[derive(PartialEq)]
pub enum Piece {
    Empty,
    WhitePawn,
    WhiteQueen,
    BlackPawn,
    BlackQueen
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Piece::BlackPawn => {
                write!{f, "🔵"}
            },
            Piece::BlackQueen => {
                write!{f, "🟦"}
            },
            Piece::Empty => {
                write!{f, "〇"}
            },
            Piece::WhitePawn => {
                write!{f, "🔴"}
            },
            Piece::WhiteQueen => {
                write!{f, "🟥"}
            },
        }
    }
}   
/// Black usually starts
pub enum Color {
    Black,
    White,
}

pub enum BoardConfig {
    Default,
    Test1,
    Test2,
    Empty,
}