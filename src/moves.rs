use std::vec;
use crate::board_setup::{Piece, Color};


/// Given an index and a board return a Vector of legal targets for that move
/// Might have to return not just scalars but also vectors again (for chain attacking)
/// Make a check for forcing moves here
pub fn get_legal_moves(board: [Piece; 32], index: usize) -> Vec<Vec<usize>> {
    if board[index] == Piece::Empty {
        // Not a valid piece 
        // Consider panicking here or returning a Result<Vec<usize>, Err> instead of just a vector
        return vec![];
    }
    match board[index] {
        Piece::BlackPawn => {return pawn_move(board, index, Color::Black)},
        Piece::BlackQueen => {return queen_move(board, index, Color::Black)},
        Piece::WhitePawn => {return pawn_move(board, index, Color::White)},
        Piece::WhiteQueen => {return queen_move(board, index, Color::White)},
        _ => {unreachable!("Unknown piece")}
    }
}

// Call this recursively perhaps
fn pawn_move(board: [Piece; 32], index: usize, color: Color) -> Vec<Vec<usize>> {
    let mut non_captures: Vec<usize> = vec![];
    let mut captures: Vec<Vec<usize>> = vec![];
    let row_idx:usize = index/4;
    let col_pos:usize = row_idx%2 + index - (row_idx*4);
    match color {
        Color::Black => {
            if index <= 3 {
                // Last row (should be promoted here)
                // legal_moves.push(vec![]);
                return captures;
            }
            else if index <= 7 {
                // second to last row (can't capture here either)
                // Move left: 1,2,3,4
                // Move right: 1,2,3
                // Left
                if board[index-4] == Piece::Empty {
                    non_captures.push(index-4);
                }
                // Right
                if index > 4 && board[index-3] == Piece::Empty{
                    non_captures.push(index-3);
                }
            }
            // has to be row 3-8
            else if index <= 31 {
                // can capture to left  on 2,3,4 
                if col_pos > 0 {
                    if board[index-9] == Piece::Empty && (board[index-5] == Piece::BlackPawn || board[index-5] == Piece::BlackQueen){
                        captures.push(vec![index-9])
                    }
                    else if board[index-5] == Piece::Empty {
                        non_captures.push(index-5);
                    }
                }
                // can capture to right on 1,2,3
                if col_pos < 3 {
                    if board[index-7] == Piece::Empty && (board[index-4] == Piece::BlackPawn || board[index-4] == Piece::BlackQueen) {
                        captures.push(vec![index-7])
                    }
                    else if board[index-4] == Piece::Empty {
                        non_captures.push(index-4);
                    }
                }

            }
            else {
                unreachable!("Index over 31");
            }
            if captures.len() > 0 {
                for (cap_idx, cap) in captures.iter().enumerate() {
                    // recursion here
                    // captures[cap_idx].push(pawn_move(board, cap[0], color));
                }
            }
        },
        Color::White => {},
    }


    return vec![];
}

fn queen_move(board: [Piece; 32], index: usize, color: Color) -> Vec<Vec<usize>> {
    return vec![];
}