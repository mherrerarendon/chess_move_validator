use crate::{Board, Position};
pub struct PositionCursor<'a> {
    board: &'a Board,
    pos_num: usize,
}

impl<'a> PositionCursor<'a> {
    pub fn new(board: &'a Board) -> PositionCursor {
        PositionCursor {
            board,
            pos_num: board.position_count(),
        }
    }

    fn generate_position(&self) -> Option<Position> {
        if self.pos_num <= self.board.position_count() && self.pos_num > 0 {
            let mut pos = Position::new();
            for piece in self.board.pieces.iter() {
                if let Some(square) = piece.curr_square() {
                    pos.insert((piece.piece, piece.white), square.clone());
                }
            }
            return Some(pos);
        }
        None
    }

    pub fn curr(&mut self) -> Position {
        self.pos_num = self.board.position_count();
        self.generate_position().unwrap()
    }

    fn next(&mut self) -> Option<Position> {
        if self.pos_num < self.board.position_count() {
            self.pos_num += 1;
            return self.generate_position();
        }
        None
    }

    fn prev(&mut self) -> Option<Position> {
        if self.pos_num > 0 {
            self.pos_num -= 1;
            return self.generate_position();
        }
        None
    }
}
