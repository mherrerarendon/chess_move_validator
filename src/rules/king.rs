use chess_pgn_parser::{Square, File, Rank};
use super::{PieceRules, PieceData, Board};
pub struct KingRules;

impl KingRules {
    pub fn new() -> Self {
        Self
    }
}

impl PieceRules for KingRules {
    fn get_initial_square(&self, piece_data: &PieceData) -> Square {
        let rank = if piece_data.white {Rank::R1} else {Rank::R8};
        Square::new_known(File::E, rank)
    }

    fn get_move_only_squares(&self, piece_data: &PieceData) -> Vec<Square> { 
        let mut squares: Vec<Square> = Vec::new();
        let curr_square = piece_data.curr_square().unwrap();
        if let Some(new_square) = curr_square.new_with_offset(0, 1) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(1, 1) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(1, 0) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(1, -1) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(0, -1) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(-1, -1) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(-1, 0) {squares.push(new_square);}
        if let Some(new_square) = curr_square.new_with_offset(-1, 1) {squares.push(new_square);}
        // TODO: add castling
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
