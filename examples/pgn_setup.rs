use chess_move_validator::{UniquePiece};

fn main() {
    let mut board = chess_move_validator::Board::new();
    if let Ok(_) = board.add_pgn_moves("1. e4 d5") {
        let _valid_squares = board.get_valid_squares_for_piece(UniquePiece::EPawn, true );
    } else {
        panic!("Unable to parse pgn moves.");
    }
}