use chess_pgn_parser::{Square, File};
use crate::rules::{PieceRules, PawnRules, RookRules, BishopRules, KingRules, KnightRules, QueenRules};

use crate::{UniquePiece};

pub struct PieceData {
    pub piece: UniquePiece,
    pub white: bool,
    pub behavior: Box<dyn PieceRules>,
    pub square_hist: Vec<Option<Square>>
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
            },
            white,
            behavior: Box::new(PawnRules::new()),
            square_hist: Vec::new()
        };
        pawn.square_hist.push(Some(pawn.behavior.get_initial_square(&pawn)));
        pawn
    }

    pub fn curr_square(&self) -> Option<&Square> {
        self.square_hist.last().expect("Should always have at least the initial square").as_ref()
    }

    pub fn move_unchecked(&mut self, square: Square) {
        self.square_hist.push(Some(square))
    }

    pub fn capture(&mut self) {
        self.square_hist.push(None)
    }

    pub fn has_moved(&self) -> bool {
        self.square_hist.len() > 1
    }
    
    pub fn new_rook(file: File, white: bool) -> Self {
        let mut rook = Self {
            piece: match file {
                File::A => UniquePiece::QRook,
                File::H => UniquePiece::KRook,
                _ => panic!("Invalid arguments for rook creation.")
            },
            white, 
            behavior: Box::new(RookRules::new()), 
            square_hist: Vec::new()
        };
        rook.square_hist.push(Some(rook.behavior.get_initial_square(&rook)));
        rook
    }

    pub fn new_knight(file: File, white: bool) -> Self {
        let mut knight = Self {
            piece: match file {
                File::B => UniquePiece::QKnight,
                File::G => UniquePiece::KKnight,
                _ => panic!("Invalid arguments for knight creation.")
            },
            white,
            behavior: Box::new(KnightRules::new()),
            square_hist: Vec::new()
        };
        knight.square_hist.push(Some(knight.behavior.get_initial_square(&knight)));
        knight
    }

    pub fn new_bishop(file: File, white: bool) -> Self {
        let mut bishop = Self {
            piece: match file {
                File::C => UniquePiece::QBishop,
                File::F => UniquePiece::KBishop,
                _ => panic!("Invalid arguments for bishop creation.")
            },
            white,
            behavior: Box::new(BishopRules::new()),
            square_hist: Vec::new()
        };
        bishop.square_hist.push(Some(bishop.behavior.get_initial_square(&bishop)));
        bishop
    }

    pub fn new_queen(white: bool) -> Self {
        let mut queen = Self {
            piece: UniquePiece::Queen,
            white,
            behavior: Box::new(QueenRules::new()),
            square_hist: Vec::new()
        };
        queen.square_hist.push(Some(queen.behavior.get_initial_square(&queen)));
        queen
    }

    pub fn new_king(white: bool) -> Self {
        let mut king = Self {
            piece: UniquePiece::King,
            white,
            behavior: Box::new(KingRules::new()),
            square_hist: Vec::new()
        };
        king.square_hist.push(Some(king.behavior.get_initial_square(&king)));
        king
    }
}
