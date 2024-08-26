use crate::board_setup::{Color, Piece};
use std::vec;

const CAPTURE_CODE: usize = 250;
const NON_CAPTURE_CODE: usize = 251;
const EMPTY_CODE: usize = 252;
const ERROR_CODE_MOVES: usize = usize::MAX;

/// Given an index and a board return a Vector of legal targets for that move
/// Might have to return not just scalars but also vectors again (for chain attacking)
/// Make a check for forcing moves here
pub fn get_legal_moves(board: &[Piece; 32], index: usize) -> Vec<usize> {
    if board[index] == Piece::Empty {
        // Not a valid piece
        // Consider panicking here or returning a Result<Vec<usize>, Err> instead of just a vector
        return vec![];
    }
    match board[index] {
        Piece::BlackPawn => return pawn_move(board, index, Color::Black, true),
        Piece::BlackQueen => return queen_move(board, index, Color::Black),
        Piece::WhitePawn => return pawn_move(board, index, Color::White, true),
        Piece::WhiteQueen => return queen_move(board, index, Color::White),
        _ => {
            unreachable!("Unknown piece")
        }
    }
}

// Call this recursively perhaps
fn pawn_move(
    board: &[Piece; 32],
    index: usize,
    player_color: Color,
    only_captures_allowed: bool,
) -> Vec<usize> {
    let mut non_captures: Vec<usize> = vec![NON_CAPTURE_CODE];
    let mut captures: Vec<usize> = vec![CAPTURE_CODE];

    let row_idx: usize = index / 4;
    let additional_increment = row_idx%2;
    let col_pos: usize = row_idx % 2 + index - (row_idx * 4);
    match player_color {
        Color::Black => {
            if index <= 3 {
                // Last row (should be promoted here)
                unreachable!("Last row without being promoted (Black)")
            } else if index <= 7 {
                // second to last row (can't capture here either)
                // Left
                if top_left_move_check(&board, &index, &additional_increment) {
                    non_captures.push(index - (4+additional_increment));
                }
                // Right
                if index > 4 && top_right_move_check(&board, &index, &additional_increment) {
                    non_captures.push(index - (additional_increment+3));
                }
            }
            // has to be row 3-8
            else if index <= 31 {
                // Captures first to end prematurely
                match col_pos {
                    // can capture to right on 1,2,3
                    // can capture to left on 2,3,4
                    0 => {
                        if top_right_capture_check(&board, &index) {
                            captures.push(index - 7);
                        }
                    }
                    1 | 2 => {
                        if top_right_capture_check(&board, &index) {
                            captures.push(index - 7);
                        }
                        if top_left_capture_check(&board, &index) {
                            captures.push(index - 9);
                        }
                    }
                    3 => {
                        if top_left_capture_check(&board, &index) {
                            captures.push(index - 9);
                        }
                    }
                    _ => {
                        unreachable!("Column position is not in 0..4")
                    }
                }
                if captures.len() > 1 {
                    // Maybe return a 250 to signal this contains captures
                    return captures;
                } else {
                    // check normal moves here
                    match index {
                        12 | 20 | 28 => {
                            // left edge
                            if top_right_move_check(&board, &index, &additional_increment) {
                                non_captures.push(index - (4+additional_increment))
                            }
                        }
                        11 | 19 | 27 => {
                            // right edge
                            if top_left_move_check(&board, &index, &additional_increment) {
                                non_captures.push(index - (3+additional_increment))
                            }
                        }
                        _ => {
                            // both dirs
                            if top_left_move_check(&board, &index, &additional_increment) {
                                non_captures.push(index-(3+additional_increment))
                            }
                            if top_right_move_check(&board, &index, &additional_increment) {
                                non_captures.push(index-(4+additional_increment))
                            }
                        }
                    }
                    // Maybe return a 251 to signal this contains captures
                    return non_captures;
                }
            } else {
                unreachable!("Index not in a feasible range");
            }
        }
        Color::White => {
            if index >= 28 {
                unreachable!("Should have promoted by now (White)")
            } else if index >= 24 {
                if bottom_left_move_check(&board, &index, &additional_increment) {
                    non_captures.push(index + 3+additional_increment)
                }
                if index < 27 && bottom_right_move_check(&board, &index, &additional_increment) {
                    non_captures.push(index + 4+additional_increment)
                }
            } else {
                match col_pos {
                    0 => {
                        if bottom_right_capture_check(&board, &index) {
                            captures.push(index + 9)
                        }
                    }
                    1 | 2 => {
                        if bottom_right_capture_check(&board, &index) {
                            captures.push(index + 9)
                        }
                        if bottom_left_capture_check(&board, &index) {
                            captures.push(index + 7)
                        }
                    }
                    3 => {
                        if bottom_left_capture_check(&board, &index) {
                            captures.push(index + 7)
                        }
                    }
                    _ => {
                        unreachable!("Weird column position")
                    }
                }
                if captures.len() > 1 {
                    // Maybe return a 250 to signal this contains captures
                    return captures;
                }
                else {
                    match index {
                        12 | 20 | 28 => {
                            // left edge
                            if bottom_right_move_check(&board, &index, &additional_increment) {
                                non_captures.push(index +3+additional_increment)
                            }
                        }
                        11 | 19 | 27 => {
                            // right edge
                            if bottom_left_move_check(&board, &index, &additional_increment) {
                                non_captures.push(index +4+additional_increment)
                            }
                        }
                        _ => {
                            // both dirs
                            if bottom_left_move_check(&board, &index, &additional_increment) {
                                non_captures.push(index+4+additional_increment)
                            }
                            if bottom_right_move_check(&board, &index, &additional_increment) {
                                non_captures.push(index+3+additional_increment)
                            }
                        }
                    }
                    return non_captures;
                }
            }
        }
    }
    return vec![ERROR_CODE_MOVES];
}

fn top_left_capture_check(board: &[Piece; 32], index: &usize) -> bool {
    if board[index - 9] == Piece::Empty
        && (board[index - 5] == Piece::BlackPawn || board[index - 5] == Piece::BlackQueen)
    {
        return true;
    }
    return false;
}

fn top_right_capture_check(board: &[Piece; 32], index: &usize) -> bool {
    if board[index - 7] == Piece::Empty
        && (board[index - 4] == Piece::BlackPawn || board[index - 4] == Piece::BlackQueen)
    {
        return true;
    }
    return false;
}

fn top_left_move_check(board: &[Piece; 32], index: &usize, increment: &usize) -> bool {
    if board[index - (4+increment)] == Piece::Empty {
        return true;
    }
    return false;
}

fn top_right_move_check(board: &[Piece; 32], index: &usize, increment: &usize) -> bool {
    if board[index - (3+increment)] == Piece::Empty {
        return true;
    }
    return false;
}

fn bottom_left_move_check(board: &[Piece; 32], index: &usize, increment: &usize) -> bool {
    if board[index + 3+increment] == Piece::Empty {
        return true;
    }
    return false;
}

fn bottom_right_move_check(board: &[Piece; 32], index: &usize, increment: &usize) -> bool {
    if board[index + 4+increment] == Piece::Empty {
        return true;
    }
    return false;
}

fn bottom_right_capture_check(board: &[Piece; 32], index: &usize) -> bool {
    if board[index + 5] == Piece::Empty
        && (board[index + 9] == Piece::WhitePawn || board[index + 9] == Piece::WhiteQueen)
    {
        return true;
    }
    return false;
}

fn bottom_left_capture_check(board: &[Piece; 32], index: &usize) -> bool {
    if board[index + 4] == Piece::Empty
        && (board[index + 7] == Piece::WhitePawn || board[index + 7] == Piece::WhiteQueen)
    {
        return true;
    }
    return false;
}

fn queen_move(board: &[Piece; 32], index: usize, color: Color) -> Vec<usize> {
    return vec![];
}
