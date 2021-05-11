use chess_pgn_parser::{File, Square, parse_moves, peggler::ParseError, Move::BasicMove};

mod rules;
mod model;

pub use rules::{UniquePiece};
use model::{PieceData};

use crate::rules::BishopRules;

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

    pub fn add_pgn_moves(&mut self, pgn_moves: &str) -> Result<(), ParseError> {
        let game_moves = parse_moves(pgn_moves)?;
        for game_move in game_moves.moves.iter() {
            match game_move.move_.move_ {
                BasicMove { piece, ref to, ref from, is_capture, promoted_to } => {
                    if let Some(piece_data) = self.get_mut_piece_data_at_square(from) {
                        piece_data.curr_square = Some(to.clone());
                        if let Some(promotion) = promoted_to {
                            match promotion {
                                chess_pgn_parser::Piece::Bishop => piece_data.behavior = Box::new(BishopRules),
                                _ => panic!("deleteme")
                            }
                        }
                    }

                    if let Some(captured_piece_data) = self.get_mut_piece_data_at_square(to) {
                        assert!(is_capture);
                        captured_piece_data.curr_square = None;
                    }
                },
                _ => panic!("deleteme")
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
