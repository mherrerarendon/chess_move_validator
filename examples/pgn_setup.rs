use chess_move_validator::{Square, UniquePiece, ChessError};

fn main() -> Result<(), ChessError> {
    let mut board = chess_move_validator::Board::new();
    
    // Setup board
    board.add_pgn_moves("1. e4 d5")?; 

    let legal_moves = board.legal_moves_from_square(&Square::E4);
    assert_eq!(legal_moves.len(), 2);

    // This should capture the black pawn on D5
    board.simple_move(&Square::E4, &Square::D5, None)?;

    let mut cursor = board.position_cursor();
    let pos = cursor.curr();
    assert_eq!(pos.get(&(UniquePiece::EPawn, true)).unwrap(), &Square::D5);
    assert!(pos.get(&(UniquePiece::DPawn, false)).is_none());
    Ok(())

}