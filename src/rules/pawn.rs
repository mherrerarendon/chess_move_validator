use chess_pgn_parser::{Square, File, Rank};
use super::{PieceRules, PieceData, UniquePiece, Board};
pub struct PawnRules;

impl PawnRules {
    pub fn new() -> Self {
        Self
    }
}

impl PieceRules for PawnRules {
    fn get_initial_square(&self, piece_data: &PieceData) -> Square {
        let rank = if piece_data.white {Rank::R2} else {Rank::R7};
        let file = match piece_data.piece {
            UniquePiece::APawn => File::A,
            UniquePiece::BPawn => File::B,
            UniquePiece::CPawn => File::C,
            UniquePiece::DPawn => File::D,
            UniquePiece::EPawn => File::E,
            UniquePiece::FPawn => File::F,
            UniquePiece::GPawn => File::G,
            UniquePiece::HPawn => File::H,
            _ => panic!("Invalid UniquePiece for pawn initial square")
        };
        Square::new_known(file, rank)
    }

    fn get_move_only_squares(&self, piece_data: &PieceData) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        let direction = if piece_data.white {1} else {-1};
        let curr_square = piece_data.curr_square.as_ref().unwrap();
        if !piece_data.has_moved {
            let double_step_square = curr_square.new_with_offset(0, 2 * direction).unwrap();
            squares.push(double_step_square);
        }

        // Always unwrap because there can't be any un-promoted pawns in the last rank
        let single_step_square = curr_square.new_with_offset(0, direction).unwrap();
        squares.push(single_step_square);

        squares
    }

    fn get_capture_only_squares(&self, piece_data: &PieceData) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        let direction = if piece_data.white {1} else {-1};
        let curr_square = piece_data.curr_square.as_ref().unwrap();

        if let Some(right_capture_square) = curr_square.new_with_offset(1, direction) {
            squares.push(right_capture_square);
        }

        if let Some(left_capture_square) = curr_square.new_with_offset(-1, direction) {
            squares.push(left_capture_square);
        }
        
        // TODO: en passant

        squares
    }

    fn get_diagonal_squares(&self, piece_data: &PieceData, board: &Board) -> Vec<Square> {
        Vec::new()
    }

    fn get_straight_squares(&self, piece_data: &PieceData, board: &Board) -> Vec<Square> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_only() {
        let pawn = PawnRules::new();

    }
}