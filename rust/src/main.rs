use checkers_rust::{board_setup, moves::{self, get_legal_moves}, visual_display};

fn main() {
    let board = board_setup::make_board(board_setup::BoardConfig::Default);
    // let mut player: setup::Color = setup::Color::Black;
    visual_display::print_board(&board);

    dbg!(get_legal_moves(&board, 20));
}
