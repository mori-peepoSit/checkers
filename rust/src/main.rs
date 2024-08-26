use checkers_rust::{board_setup, visual_display};

fn main() {
    let board = board_setup::make_board(board_setup::BoardConfig::Default);
    // let mut player: setup::Color = setup::Color::Black;
    visual_display::print_board(board);

    for i in 0..32 {
        print!("{} ", visual_display::convert_idx_to_coords(i));
    }
}
