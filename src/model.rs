use chess_pgn_parser::{Square, File};
use super::{UniquePiece};
use super::rules::{PieceRules, PawnRules};

pub struct PieceData {
    pub piece: UniquePiece,
    pub white: bool,
    pub curr_square: Option<Square>,
    behavior: Box<dyn PieceRules>,
    pub has_moved: bool
}

impl PieceData {
    pub fn new_pawn(file: File, white: bool) -> Self {
        let mut pawn = Self {
            piece: match file {
                File::A => UniquePiece::APawn,
                File::B => UniquePiece::BPawn,
                File::C => UniquePiece::CPawn,
                File::D => UniquePiece::DPawn,
                File::E => UniquePiece::EPawn,
                File::F => UniquePiece::FPawn,
                File::G => UniquePiece::GPawn,
                File::H => UniquePiece::HPawn,
                _ => panic!("Invalid arguments for pawn creation")
            },
            white,
            curr_square: None,
            behavior: Box::new(PawnRules::new()),
            has_moved: false
        };
        pawn.curr_square = Some(pawn.behavior.get_initial_square(&pawn));
        pawn
    }
}
