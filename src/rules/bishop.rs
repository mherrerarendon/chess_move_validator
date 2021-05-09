use chess_pgn_parser::{Square, File, Rank};
use super::{PieceRules, PieceData, UniquePiece, Board};
pub struct BishopRules;

impl BishopRules {
    pub fn new() -> Self {
        Self
    }
}

impl PieceRules for BishopRules {
    fn get_initial_square(&self, piece_data: &PieceData) -> Square {
        let rank = if piece_data.white {Rank::R1} else {Rank::R8};
        let file = match piece_data.piece {
            UniquePiece::QBishop => File::C,
            UniquePiece::KBishop => File::F,
            _ => panic!("Invalid UniquePiece for pawn initial square")
        };
        Square::new_known(file, rank)
    }

    fn get_straight_squares(&self, _piece_data: &PieceData, _board: &Board) -> Vec<Square> {
        Vec::new()
    }
}

