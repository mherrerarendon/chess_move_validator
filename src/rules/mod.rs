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
                match board.get_piece_data_at_square(&new_square) {
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
            } else {
                break;
            }
        }

        squares
    }

    fn get_valid_squares(&self, piece_data: &PieceData, board: &Board) -> Vec<Square> {
        let mut squares = self.get_diagonal_squares(piece_data, board);
        squares.extend(self.get_straight_squares(piece_data, board));
        let move_only_squares: Vec<Square> = self.get_move_only_squares(piece_data).into_iter()
            .filter(|square| board.get_piece_data_at_square(square).is_none()).collect();

        let capture_only_squares: Vec<Square> = self.get_capture_only_squares(piece_data).into_iter()
            .filter(|square| {
                if let Some(piece) = board.get_piece_data_at_square(square) {
                    return piece.white != piece_data.white;
                }

                return false;
            }).collect();

        squares.extend(move_only_squares.into_iter());
        squares.extend(capture_only_squares.into_iter());

        squares
    }
}

#[cfg(test)]
mod tests {
    use chess_pgn_parser::Square;
    use crate::{Board, UniquePiece};

    #[test]
    fn test_pawn_behavior() {
        let board = Board::new();     
        let pawn = board.get_piece_data_at_square(&Square::A2).expect("mising piece.");
        assert_eq!(pawn.piece, UniquePiece::APawn);
        let valid_squares = pawn.behavior.get_valid_squares(pawn, &board);
        assert_eq!(2, valid_squares.len());
    }
    
    #[test]
    fn test_rook_behavior() {
        let board = Board::new();
        let rook = board.get_piece_data_at_square(&Square::A1).expect("missing piece.");
        assert_eq!(rook.piece, UniquePiece::QRook);
        let valid_squares = rook.behavior.get_valid_squares(rook, &board);
        assert_eq!(0, valid_squares.len());
    }
    
    #[test]
    fn test_knight_behavior() {
        let board = Board::new();
        let knight = board.get_piece_data_at_square(&Square::B1).expect("missing piece.");
        assert_eq!(knight.piece, UniquePiece::QKnight);
        let valid_squares = knight.behavior.get_valid_squares(knight, &board);
        assert_eq!(2, valid_squares.len());
    }

    #[test]
    fn test_bishop_behavior() {
        let board = Board::new();
        let bishop = board.get_piece_data_at_square(&Square::C1).expect("missing piece.");
        assert_eq!(bishop.piece, UniquePiece::QBishop);
        let valid_squares = bishop.behavior.get_valid_squares(bishop, &board);
        assert_eq!(0, valid_squares.len());
    }

    #[test]
    fn test_queen_behavior() {
        let board = Board::new();
        let queen = board.get_piece_data_at_square(&Square::D1).expect("missing piece.");
        assert_eq!(queen.piece, UniquePiece::Queen);
        let valid_squares = queen.behavior.get_valid_squares(queen, &board);
        assert_eq!(0, valid_squares.len());
    }

    #[test]
    fn test_king_behavior() {
        let board = Board::new();
        let king = board.get_piece_data_at_square(&Square::E1).expect("missing piece.");
        assert_eq!(king.piece, UniquePiece::King);
        let valid_squares = king.behavior.get_valid_squares(king, &board);
        assert_eq!(0, valid_squares.len());
    }
}
