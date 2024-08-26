use crate::board_setup;

pub fn print_board(board: [board_setup::Piece; 32]) {
    println!("");
    let mut start_of_row: usize;
    for row_idx in 0..8 {
        start_of_row = row_idx * 4;
        print!("{}| ", row_idx+1);
        match row_idx%2 {
            0 => {
                // Even row (starts on the left with an empty space)
                print!(" ・{} ・{} ・{} ・{}", board[start_of_row], board[start_of_row+1], board[start_of_row+2], board[start_of_row+3]);
            }
            1 => {
                // Uneven row (starts on the left with a piece directly)
                print!("{} ・{} ・{} ・{} ・", board[start_of_row], board[start_of_row+1], board[start_of_row+2], board[start_of_row+3]);
            }
            _ => {unreachable!()}
            } 
        println!();
    }
}

/// let test: char = char::from_u32(65).unwrap();
/// for the reverse of this function
fn convert_char_to_ascii(character: char) -> u8 {
    return character.to_ascii_lowercase() as u8;
}


pub fn convert_idx_to_coords(idx: usize) -> String {
    let mut output: String = String::new();
    let row_idx = idx/4 as usize;
    let letter_idx: usize = idx - row_idx*4;
    // println!("idx{} row{}  letter{}", idx, row_idx, letter_idx);
    match row_idx%2 {
        1 => {
            match letter_idx {
                0 => {
                    output += "a";
                },
                1 => {
                    output += "c";
                },
                2 => {
                    output += "e";
                },
                3 => {
                    output += "g";
                },
                _ => {unreachable!()}
            }
        }
        0 => {
            match letter_idx {
                0 => {
                    output += "b";
                },
                1 => {
                    output += "d";
                },
                2 => {
                    output += "f";
                },
                3 => {
                    output += "h";
                },
                _ => {unreachable!()}
            }
        }
        _ => {}
    }
    output += &(row_idx+1).to_string();
    return output;
}