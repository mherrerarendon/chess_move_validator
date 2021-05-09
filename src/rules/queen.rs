use chess_pgn_parser::{Square, File, Rank};
use super::{PieceRules, PieceData};
pub struct QueenRules;

impl QueenRules {
    pub fn new() -> Self {
        Self
    }
}

impl PieceRules for QueenRules {
    fn get_initial_square(&self, piece_data: &PieceData) -> Square {
        let rank = if piece_data.white {Rank::R1} else {Rank::R8};
        Square::new_known(File::D, rank)
    }
}
