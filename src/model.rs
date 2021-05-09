use chess_pgn_parser::{Square, File};
use crate::rules::{PieceRules, PawnRules, RookRules, BishopRules, KingRules, KnightRules, QueenRules};

use crate::{UniquePiece};

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
    
    pub fn new_rook(file: File, white: bool) -> Self {
        let mut rook = Self {
            piece: match file {
                File::A => UniquePiece::QRook,
                File::H => UniquePiece::KRook,
                _ => panic!("Invalid arguments for rook creation.")
            },
            white, 
            curr_square: None,
            behavior: Box::new(RookRules::new()), 
            has_moved: false
        };
        rook.curr_square = Some(rook.behavior.get_initial_square(&rook));
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
            curr_square: None,
            behavior: Box::new(KnightRules::new()),
            has_moved: false
        };
        knight.curr_square = Some(knight.behavior.get_initial_square(&knight));
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
            curr_square: None,
            behavior: Box::new(BishopRules::new()),
            has_moved: false
        };
        bishop.curr_square = Some(bishop.behavior.get_initial_square(&bishop));
        bishop
    }

    pub fn new_queen(white: bool) -> Self {
        let mut queen = Self {
            piece: UniquePiece::Queen,
            white,
            curr_square: None,
            behavior: Box::new(QueenRules::new()),
            has_moved: false
        };
        queen.curr_square = Some(queen.behavior.get_initial_square(&queen));
        queen
    }

    pub fn new_king(white: bool) -> Self {
        let mut king = Self {
            piece: UniquePiece::King,
            white,
            curr_square: None,
            behavior: Box::new(KingRules::new()),
            has_moved: false
        };
        king.curr_square = Some(king.behavior.get_initial_square(&king));
        king
    }
}
