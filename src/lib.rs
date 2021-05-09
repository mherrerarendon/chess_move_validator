use chess_pgn_parser::{Square, File, Rank};

mod rules;
mod model;

pub use rules::{UniquePiece};
use model::{PieceData};

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
        vec![PieceData::new_pawn(File::A, true)]
    }

    pub fn add_pgn_moves(&mut self, pgn_moves: &str) {

    }

    pub fn get_valid_squares_for_piece(&self, piece: UniquePiece, white: bool) -> Vec<Square> {
        vec![Square::E1]
    }

    pub(crate) fn get_piece_at_square(&self, square: &Square) -> Option<&PieceData> {
        for piece in self.pieces.iter() {
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
