use chess_pgn_parser::{File, Move::BasicMove, Move::CastleKingside, Move::CastleQueenside, Piece, Rank, Square, parse_moves, peggler::ParseError};

mod rules;
mod model;

pub use rules::{UniquePiece};
use model::{PieceData};

use crate::rules::{BishopRules, KnightRules, QueenRules, RookRules};

pub struct Board {
    pieces: Vec<PieceData>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: Self::create_initial_pieces()
        }
    }

    fn create_initial_pieces() -> Vec<PieceData> {
        vec![
            PieceData::new_pawn(File::A, true),
            PieceData::new_pawn(File::B, true),
            PieceData::new_pawn(File::C, true),
            PieceData::new_pawn(File::D, true),
            PieceData::new_pawn(File::E, true),
            PieceData::new_pawn(File::F, true),
            PieceData::new_pawn(File::G, true),
            PieceData::new_pawn(File::H, true),
            PieceData::new_pawn(File::A, false),
            PieceData::new_pawn(File::B, false),
            PieceData::new_pawn(File::C, false),
            PieceData::new_pawn(File::D, false),
            PieceData::new_pawn(File::E, false),
            PieceData::new_pawn(File::F, false),
            PieceData::new_pawn(File::G, false),
            PieceData::new_pawn(File::H, false),

            PieceData::new_rook(File::A, true),
            PieceData::new_rook(File::H, true),
            PieceData::new_rook(File::A, false),
            PieceData::new_rook(File::H, false),

            PieceData::new_knight(File::B, true),
            PieceData::new_knight(File::G, true),
            PieceData::new_knight(File::B, false),
            PieceData::new_knight(File::G, false),

            PieceData::new_bishop(File::C, true),
            PieceData::new_bishop(File::F, true),
            PieceData::new_bishop(File::C, false),
            PieceData::new_bishop(File::F, false),

            PieceData::new_queen(true),
            PieceData::new_queen(false),

            PieceData::new_king(true),
            PieceData::new_king(false),
        ]
    }

    fn disambiguate_from_square(&self, piece: Piece, to: &Square) -> Square {
        todo!("implement");
    }

    pub fn add_pgn_moves(&mut self, pgn_moves: &str) -> Result<(), ParseError> {
        let game_moves = parse_moves(pgn_moves)?;
        for game_move in game_moves.moves.iter() {
            match game_move.move_.move_ {
                BasicMove { piece, ref to, ref from, is_capture, promoted_to } => {
                    let known_from = match from.get_known() {
                        Some(known_from) => known_from,
                        None => {
                            self.disambiguate_from_square(piece, to)
                        }
                    };
                    let piece_data = self.get_mut_piece_data_at_square(&known_from).expect("Missing piece");
                    piece_data.curr_square = Some(to.clone());
                    piece_data.has_moved = true;
                    if let Some(promotion) = promoted_to {
                        match promotion {
                            chess_pgn_parser::Piece::Rook => piece_data.behavior = Box::new(RookRules),
                            chess_pgn_parser::Piece::Knight => piece_data.behavior = Box::new(KnightRules),
                            chess_pgn_parser::Piece::Bishop => piece_data.behavior = Box::new(BishopRules),
                            chess_pgn_parser::Piece::Queen => piece_data.behavior = Box::new(QueenRules),
                            _ => panic!("Invalid promotion")
                        }
                    }
                    
                    let captured_piece_data = self.get_mut_piece_data_at_square(to).expect("Missing piece");
                    assert!(is_capture);
                    captured_piece_data.curr_square = None;
                },
                ref c @ CastleKingside | ref c @ CastleQueenside => {
                    let rank = if game_move.number.is_some() {Rank::R8} else {Rank::R1};
                    let (old_rook_file, new_king_file, new_rook_file) = match c {
                        CastleKingside => (File::H, File::G, File::F),
                        CastleQueenside => (File::A, File::C, File::D),
                        _ => unreachable!()
                    };
                    let old_king_square = Square::new_known(File::E, rank);
                    let new_king_square = Square::new_known(new_king_file, rank);
                    let old_rook_square = Square::new_known(old_rook_file, rank);
                    let new_rook_square = Square::new_known(new_rook_file, rank);
                    let king_piece_data = self.get_mut_piece_data_at_square(&old_king_square).expect("Missing piece");
                    king_piece_data.curr_square = Some(new_king_square);
                    king_piece_data.has_moved = true;
                    
                    let rook_data = self.get_mut_piece_data_at_square(&old_rook_square).expect("Missing piece");
                    rook_data.curr_square = Some(new_rook_square);
                    rook_data.has_moved = true;
                }
            }
        }
        Ok(())
    }

    pub fn get_valid_squares_for_piece(&self, piece: UniquePiece, white: bool) -> Vec<Square> {
        vec![Square::E1]
    }

    pub(crate) fn get_piece_data_at_square(&self, square: &Square) -> Option<&PieceData> {
        for piece in self.pieces.iter() {
            if let Some(ref curr_square) = piece.curr_square {
                if curr_square == square {
                    return Some(piece);
                }
            }
        }
        return None;
    }
    
    pub(crate) fn get_mut_piece_data_at_square(&mut self, square: &Square) -> Option<&mut PieceData> {
        for piece in self.pieces.iter_mut() {
            if let Some(ref curr_square) = piece.curr_square {
                if curr_square == square {
                    return Some(piece);
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
