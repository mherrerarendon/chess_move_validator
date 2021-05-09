use chess_pgn_parser::{Square};
use crate::model::{PieceData};
use super::Board;

pub mod pawn;
pub mod rook;
pub mod knight;
pub mod bishop;
pub mod queen;
pub mod king;

pub use pawn::PawnRules as PawnRules;
pub use rook::RookRules as RookRules;
pub use knight::KnightRules as KnightRules;
pub use bishop::BishopRules as BishopRules;
pub use queen::QueenRules as QueenRules;
pub use king::KingRules as KingRules;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UniquePiece {
    APawn, BPawn, CPawn, DPawn, EPawn, FPawn, GPawn, HPawn,
    QRook, QKnight, QBishop, Queen, King, KBishop, KKnight, KRook
}

pub trait PieceRules {
    fn get_initial_square(&self, piece_data: &PieceData) -> Square;
    fn get_move_only_squares(&self, _piece_data: &PieceData) -> Vec<Square> { Vec::new() }
    fn get_capture_only_squares(&self, _piece_data: &PieceData) -> Vec<Square> { Vec::new() }
    fn get_single_move_or_capture_squares(&self, _piece_data: &PieceData) -> Vec<Square> { Vec::new() }

    // Override with empty square Vec to disable
    fn get_diagonal_squares(&self, piece_data: &PieceData, board: &Board) -> Vec<Square> {
        let mut squares = self.get_linear_squares_with_offsets(piece_data, board, 1, 1);
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, 1, -1));
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, -1, -1));
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, -1, 1));

        squares
    }
    
    // Override with empty square Vec to disable
    fn get_straight_squares(&self, piece_data: &PieceData, board: &Board) -> Vec<Square> {
        let mut squares = self.get_linear_squares_with_offsets(piece_data, board, 0, 1);
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, 0, -1));
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, 1, 0));
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, -1, 0));

        squares
    }
    
    fn get_linear_squares_with_offsets(&self, piece_data: &PieceData, board: &Board, file_offset: i32, rank_offset: i32) -> Vec<Square> {
        let mut squares = Vec::new();
        let mut curr_square = piece_data.curr_square.as_ref().unwrap().clone();
        loop {
            if let Some(new_square) = curr_square.new_with_offset(file_offset, rank_offset) {
                match board.get_piece_at_square(&new_square) {
                    Some(piece_at_square) if piece_at_square.white != piece_data.white => {
                        squares.push(new_square);
                        break;
                    },
                    None => {
                        curr_square = new_square.clone();
                        squares.push(new_square);
                    },
                    _ => break
                }
            }
        }

        squares
    }

    fn get_valid_squares(&self, piece_data: &PieceData, board: &Board) -> Vec<Square> {
        let mut squares = self.get_diagonal_squares(piece_data, board);
        squares.extend(self.get_straight_squares(piece_data, board));
        let move_only_squares: Vec<Square> = self.get_move_only_squares(piece_data).into_iter()
            .filter(|square| board.get_piece_at_square(square).is_none()).collect();

        let capture_only_squares: Vec<Square> = self.get_capture_only_squares(piece_data).into_iter()
            .filter(|square| {
                if let Some(piece) = board.get_piece_at_square(square) {
                    return piece.white != piece_data.white;
                }

                return false;
            }).collect();

        squares.extend(move_only_squares.into_iter());
        squares.extend(capture_only_squares.into_iter());

        squares
    }
}
