use chess_move_validator::{Board, UniquePiece};
use chess_pgn_parser::{Piece};

fn main() {
    let mut board = chess_move_validator::Board::new();
    board.add_pgn_moves("1. e4 d5");
    let valid_squares = board.get_valid_squares_for_piece(UniquePiece::EPawn, true /* white */);

}