use chess_pgn_parser::{Square, File, Rank};
use super::{PieceRules, PieceData, UniquePiece, Board};
pub struct KnightRules;

impl KnightRules {
    pub fn new() -> Self {
        Self
    }
}

impl PieceRules for KnightRules {
    fn get_initial_square(&self, piece_data: &PieceData) -> Square {
        let rank = if piece_data.white {Rank::R1} else {Rank::R8};
        let file = match piece_data.piece {
            UniquePiece::QKnight => File::B,
            UniquePiece::KKnight => File::G,
            _ => panic!("Invalid UniquePiece for pawn initial square")
        };
        Square::new_known(file, rank)
    }

    fn get_move_only_squares(&self, piece_data: &PieceData) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        let curr_square = piece_data.curr_square().unwrap();
        if let Some(new_square) = curr_square.new_with_offset(1, 2) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(2, 1) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(2, -1) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(1, -2) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(-1, -2) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(-2, -1) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(-2, 1) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(-1, 2) {squares.push(new_square);}
        squares
    }

    fn get_capture_only_squares(&self, piece_data: &PieceData) -> Vec<Square> {
        self.get_move_only_squares(piece_data)
    }

    fn get_diagonal_squares(&self, _piece_data: &PieceData, _board: &Board) -> Vec<Square> {
        Vec::new()
    }

    fn get_straight_squares(&self, _piece_data: &PieceData, _board: &Board) -> Vec<Square> {
        Vec::new()
    }
}
