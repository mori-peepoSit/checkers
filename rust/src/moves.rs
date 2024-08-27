use crate::board_setup::{Color, Piece};
    use std::vec;

const CAPTURE_CODE: u8 = 250;
const NON_CAPTURE_CODE: u8 = 251;
const EMPTY_CODE: u8 = 252;
const ERROR_CODE_MOVES: u8 = u8::MAX;
const SEPARATOR_VALUE: u8 = 240;

/// Given an index and a board return a Vector of legal targets for that move
/// Might have to return not just scalars but also vectors again (for chain attacking)
/// Make a check for forcing moves here
pub fn get_legal_moves(board: &[Piece; 32], index: u8) -> Vec<u8> {
    if board[index as usize] == Piece::Empty {
        // Not a valid piece
        // Consider panicking here or returning a Result<Vec<u8>, Err> instead of just a vector
        return vec![EMPTY_CODE];
    }
    let mut moves: Vec<u8>;
    match board[index as usize] {
        Piece::BlackPawn => {
            moves =  pawn_move(board, index, &Color::Black);
        },
        Piece::BlackQueen => return queen_move(board, index, Color::Black),
        Piece::WhitePawn => {
            moves =  pawn_move(board, index, &Color::White);
        },
        Piece::WhiteQueen => return queen_move(board, index, Color::White),
        _ => {
            unreachable!("Unknown piece")
        }
    }
    let code  = moves.swap_remove(0);
    if moves.len() == 0 {
        moves.push(EMPTY_CODE);
    }
    match code {
        CAPTURE_CODE => {
            // recurse
            return moves;
        },
        NON_CAPTURE_CODE => {
            return moves;
        },
        EMPTY_CODE => {
            return moves;
        }
        ERROR_CODE_MOVES => {
            return moves;
        },
        _ => {unreachable!("no code provided after getting moves ?")}
    }
}

// Call this recursively perhaps
fn pawn_move(
    board: &[Piece; 32],
    index: u8,
    player_color: &Color,
) -> Vec<u8> {
    let mut non_captures: Vec<u8> = vec![NON_CAPTURE_CODE];
    let mut captures: Vec<u8> = vec![CAPTURE_CODE];
    let mut capture_chain: Vec<u8> = vec![CAPTURE_CODE];

    match player_color {
        Color::Black => {
            match index {
                4 => {
                    // first field
                    if board[0] == Piece::Empty {
                        non_captures.push(0);
                    }
                }
                5 | 6 | 7 => {
                    // top row
                    if board[(index - 5) as usize] == Piece::Empty {
                        non_captures.push(index - 5)
                    }
                    if board[(index - 4) as usize] == Piece::Empty {
                        non_captures.push(index - 4)
                    }
                }
                8 | 16 | 24 => {
                    // 2nd from left
                    match board[(index - 3) as usize] {
                        Piece::Empty => {
                            non_captures.push(index - 3);
                            if board[(index - 4) as usize] == Piece::Empty {
                                non_captures.push(index - 4);
                            }
                        }
                        Piece::WhitePawn | Piece::WhiteQueen => {
                            if board[(index - 7) as usize] == Piece::Empty {
                                captures.push(index - 7);
                            }
                        }
                        _ => {}
                    }
                }
                9 | 10 | 17 | 18 | 25 | 26 => {
                    // Middle short
                    // move+cap left+right (-4 -3)
                    match board[(index - 4) as usize] {
                        Piece::Empty => {
                            non_captures.push(index - 4);
                        }
                        Piece::WhitePawn | Piece::WhiteQueen => {
                            if board[(index - 9) as usize] == Piece::Empty {
                                captures.push(index - 9);
                            }
                        }
                        _ => {}
                    }
                    match board[(index - 3) as usize] {
                        Piece::Empty => {
                            non_captures.push(index - 3);
                        }
                        Piece::WhitePawn | Piece::WhiteQueen => {
                            if board[(index - 7) as usize] == Piece::Empty {
                                captures.push(index - 7);
                            }
                        }
                        _ => {}
                    }
                    

                }
                12 | 20 | 28 => {
                    // Left edge
                    match board[(index - 4) as usize] {
                        Piece::Empty => {
                            non_captures.push(index - 4);
                        }
                        Piece::WhitePawn | Piece::WhiteQueen => {
                            if board[(index - 7) as usize] == Piece::Empty {
                                captures.push(index - 7);
                            }
                        }
                        _ => {}
                    }
                }
                11 | 19 | 27 => {
                    // right edge
                    match board[(index - 4) as usize] {
                        Piece::Empty => {
                            non_captures.push(index - 4);
                        }
                        Piece::WhitePawn | Piece::WhiteQueen => {
                            if board[(index - 9) as usize] == Piece::Empty {
                                captures.push(index - 9);
                            }
                        }
                        _ => {}
                    }
                }
                13 | 14 | 21 | 22 | 29 | 30 => {
                    // Middle long
                    // (-5 -4)
                    match board[(index - 5) as usize] {
                        Piece::Empty => {
                            non_captures.push(index - 5);
                        }
                        Piece::WhitePawn | Piece::WhiteQueen => {
                            if board[(index - 9) as usize] == Piece::Empty {
                                captures.push(index - 9);
                            }
                        }
                        _ => {}
                    }
                    match board[(index - 4) as usize] {
                        Piece::Empty => {
                            non_captures.push(index - 4);
                        }
                        Piece::WhitePawn | Piece::WhiteQueen => {
                            if board[(index - 7) as usize] == Piece::Empty {
                                captures.push(index - 7);
                            }
                        }
                        _ => {}
                    }
                }
                15 | 23 | 31 => {
                    // 2nd from right
                    match board[(index - 5) as usize] {
                        Piece::Empty => {
                            non_captures.push(index - 5);
                            if board[(index - 4) as usize] == Piece::Empty {
                                non_captures.push(index - 4);
                            }
                        }
                        Piece::WhitePawn | Piece::WhiteQueen => {
                            if board[(index - 9) as usize] == Piece::Empty {
                                captures.push(index - 9);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    unreachable!("Shouldnt be here ({})", index)
                }
            }
        }
        Color::White => {
            match index {
                27 => {
                    // first field
                    if board[31] == Piece::Empty {
                        non_captures.push(31);
                    }
                }
                24 | 25 | 26 => {
                    // bottom row
                    if board[(index + 5) as usize] == Piece::Empty {
                        non_captures.push(index + 5)
                    }
                    if board[(index + 4) as usize] == Piece::Empty {
                        non_captures.push(index + 4)
                    }
                }
                8 | 16 | 0 => {
                    // 2nd from left
                    match board[(index + 5) as usize] {
                        Piece::Empty => {
                            non_captures.push(index + 5);
                            if board[(index + 4) as usize] == Piece::Empty {
                                non_captures.push(index + 4);
                            }
                        }
                        Piece::BlackPawn | Piece::BlackQueen => {
                            if board[(index + 9) as usize] == Piece::Empty {
                                captures.push(index + 9);
                            }
                        }
                        _ => {}
                    }
                }
                9 | 10 | 17 | 18 | 1 | 2 => {
                    // Middle Long
                    // move+cap left+right (+4 +5)
                    match board[(index +4) as usize] {
                        Piece::Empty => {
                            non_captures.push(index +4);
                        }
                        Piece::BlackPawn | Piece::BlackQueen  => {
                            if board[(index +7) as usize] == Piece::Empty {
                                captures.push(index +7);
                            }
                        }
                        _ => {}
                    }
                    match board[(index +5) as usize] {
                        Piece::Empty => {
                            non_captures.push(index +5);
                        }
                        Piece::BlackPawn | Piece::BlackQueen => {
                            if board[(index +9) as usize] == Piece::Empty {
                                captures.push(index +9);
                            }
                        }
                        _ => {}
                    }
                }
                12 | 20 | 4 => {
                    // Left edge
                    match board[(index + 4) as usize] {
                        Piece::Empty => {
                            non_captures.push(index + 4);
                        }
                        Piece::BlackPawn | Piece::BlackQueen  => {
                            if board[(index + 9) as usize] == Piece::Empty {
                                captures.push(index + 9);
                            }
                        }
                        _ => {}
                    }
                }
                11 | 19 | 3 => {
                    // right edge
                    match board[(index +4) as usize] {
                        Piece::Empty => {
                            non_captures.push(index + 4);
                        }
                        Piece::BlackPawn | Piece::BlackQueen  => {
                            if board[(index +7) as usize] == Piece::Empty {
                                captures.push(index +7);
                            }
                        }
                        _ => {}
                    }
                }
                13 | 14 | 21 | 22 | 5 | 6 => {
                    // Middle long
                    // (+3 +4)
                    match board[(index + 4) as usize] {
                        Piece::Empty => {
                            non_captures.push(index +4);
                        }
                        Piece::BlackPawn | Piece::BlackQueen  => {
                            if board[(index + 9) as usize] == Piece::Empty {
                                captures.push(index + 9);
                            }
                        }
                        _ => {}
                    }
                    match board[(index +3) as usize] {
                        Piece::Empty => {
                            non_captures.push(index +3);
                        }
                        Piece::BlackPawn | Piece::BlackQueen  => {
                            if board[(index + 7) as usize] == Piece::Empty {
                                captures.push(index + 7);
                            }
                        }
                        _ => {}
                    }
                }
                15 | 23 | 7 => {
                    // 2nd from right
                    match board[(index +3) as usize] {
                        Piece::Empty => {
                            non_captures.push(index +3);
                            if board[(index + 4) as usize] == Piece::Empty {
                                non_captures.push(index + 4);
                            }
                        }
                        Piece::BlackPawn | Piece::BlackQueen  => {
                            if board[(index +7) as usize] == Piece::Empty {
                                captures.push(index +7);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    unreachable!("Shouldnt be here")
                }
            }
        }
    }
    if captures.len() > 1 {
        for cap in captures {
            if  cap > 31 {
                continue;
            }
            capture_chain.push(cap);
            let recursive_return_value = &mut pawn_move(board, cap, player_color);
            if (*recursive_return_value).len() > 1 && recursive_return_value[0] == CAPTURE_CODE {
                capture_chain.append(recursive_return_value);
            }
        }
        return capture_chain;
    }
    else if non_captures.len() > 1 {
        return non_captures;
    }
    else {
        return vec![ERROR_CODE_MOVES];
    }
}

fn queen_move(board: &[Piece; 32], index: u8, color: Color) -> Vec<u8> {
    return vec![];
}
