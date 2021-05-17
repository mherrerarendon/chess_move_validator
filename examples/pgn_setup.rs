use chess_move_validator::Square;
use chess_pgn_parser::peggler::ParseError;

fn main() -> Result<(), ParseError> {
    let mut board = chess_move_validator::Board::new();
    
    // Setup board
    board.add_pgn_moves("1. e4 d5")?; 

    let legal_moves = board.legal_moves_from_square(&Square::E4);
    assert_eq!(legal_moves.len(), 2);
    Ok(())
}