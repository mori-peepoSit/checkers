#[cfg(not(feature = "alternate_symbols"))]
const WHITE_PAWN_SYMBOL: &str = "白";
#[cfg(not(feature = "alternate_symbols"))]
const WHITE_QUEEN_SYMBOL: &str = "姫";
#[cfg(not(feature = "alternate_symbols"))]
const EMPTY_SYMBOL: &str = "〇";
#[cfg(not(feature = "alternate_symbols"))]
const BLACK_PAWN_SYMBOL: &str = "黒";
#[cfg(not(feature = "alternate_symbols"))]
const BLACK_QUEEN_SYMBOL: &str = "嬢";

#[cfg(feature = "alternate_symbols")]
const WHITE_PAWN_SYMBOL: &str = "🔴";
#[cfg(feature = "alternate_symbols")]
const WHITE_QUEEN_SYMBOL: &str = "🟥";
#[cfg(feature = "alternate_symbols")]
const EMPTY_SYMBOL: &str = "〇";
#[cfg(feature = "alternate_symbols")]
const BLACK_PAWN_SYMBOL: &str = "🔵";
#[cfg(feature = "alternate_symbols")]
const BLACK_QUEEN_SYMBOL: &str = "🟦";





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
        BoardConfig::CaptureTest => {
            return [
                Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn,
                Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn,
                Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn, Piece::WhitePawn,
                Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, 
                Piece::Empty, Piece::WhitePawn, Piece::Empty, Piece::Empty, 
                Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, 
                Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, 
                Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, 
                ];
        },
        BoardConfig::ChainCaptureTest => {
            return [
                Piece::Empty,     Piece::Empty,     Piece::Empty,     Piece::Empty, 
                Piece::Empty,     Piece::Empty,     Piece::Empty,     Piece::Empty, 
                Piece::Empty,     Piece::WhitePawn, Piece::WhitePawn, Piece::Empty, 
                Piece::Empty,     Piece::Empty,     Piece::Empty,     Piece::Empty, 
                Piece::Empty,     Piece::WhitePawn, Piece::WhitePawn, Piece::Empty, 
                Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, 
                Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, 
                Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, Piece::BlackPawn, 
                ];
        }
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
                write!{f, "{}", BLACK_PAWN_SYMBOL}
            },
            Piece::BlackQueen => {
                write!{f, "{}", BLACK_QUEEN_SYMBOL}
            },
            Piece::Empty => {
                write!{f, "{}", EMPTY_SYMBOL}
            },
            Piece::WhitePawn => {
                write!{f, "{}", WHITE_PAWN_SYMBOL}
            },
            Piece::WhiteQueen => {
                write!{f, "{}", WHITE_QUEEN_SYMBOL}
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
    CaptureTest,
    ChainCaptureTest,
    Empty,
}