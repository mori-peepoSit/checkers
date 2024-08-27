use checkers_rust::{board_setup, moves, visual_display};

fn main() {
    let board = board_setup::make_board(board_setup::BoardConfig::ChainCaptureTest2);
    // let mut player: setup::Color = setup::Color::Black;
    visual_display::print_board(&board);

    for i in 21..23 {
        dbg!(i, &board[i as usize], moves::get_legal_moves(&board, i));
    }
}
