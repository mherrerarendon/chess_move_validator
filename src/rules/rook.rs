use chess_pgn_parser::{Square, File, Rank};
use super::{PieceRules, PieceData, UniquePiece, Board};
pub struct RookRules;

impl RookRules {
    pub fn new() -> Self {
        Self
    }
}

impl PieceRules for RookRules {
    fn get_initial_square(&self, piece_data: &PieceData) -> Square {
        let rank = if piece_data.white {Rank::R1} else {Rank::R8};
        let file = match piece_data.piece {
            UniquePiece::QRook => File::A,
            UniquePiece::KRook => File::H,
            _ => panic!("Invalid UniquePiece for pawn initial square")
        };
        Square::new_known(file, rank)
    }

    fn get_diagonal_squares(&self, _piece_data: &PieceData, _board: &Board) -> Vec<Square> {
        Vec::new()
    }
}
