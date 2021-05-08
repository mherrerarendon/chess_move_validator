use chess_pgn_parser::{Square, File, Rank};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UniquePiece {
    APawn, BPawn, CPawn, DPawn, EPawn, FPawn, GPawn, HPawn,
    QRook, QKnight, QBishop, Queen, King, KBishop, KKnight, KRook
}

struct PieceData {
    piece: UniquePiece,
    white: bool,
    curr_square: Option<Square>,
    behavior: Box<dyn PieceRules>,
    has_moved: bool
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

trait PieceRules {
    fn get_initial_square(&self, piece_data: &PieceData) -> Square;
    fn get_move_only_squares(&self, piece_data: &PieceData) -> Vec<Square> { Vec::new() }
    fn get_capture_only_squares(&self, piece_data: &PieceData) -> Vec<Square> { Vec::new() }
    fn get_single_move_or_capture_squares(&self, piece_data: &PieceData) -> Vec<Square> { Vec::new() }

    // Override with empty square Vec to disable
    fn get_diagonal_squares(&self, piece_data: &PieceData, board: &Board) -> Vec<Square> {
        let mut squares = self.get_linear_squares_with_offsets(piece_data, board, 1, 1);
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, 1, -1));
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, -1, -1));
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, -1, 1));

        squares
    }
    
    // Override with empty square Vec to disable
    fn get_straight_squares(&self, piece_data: &PieceData, board: &Board) -> Vec<Square> {
        let mut squares = self.get_linear_squares_with_offsets(piece_data, board, 0, 1);
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, 0, -1));
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, 1, 0));
        squares.extend(self.get_linear_squares_with_offsets(piece_data, board, -1, 0));

        squares
    }
    
    fn get_linear_squares_with_offsets(&self, piece_data: &PieceData, board: &Board, file_offset: i32, rank_offset: i32) -> Vec<Square> {
        let mut squares = Vec::new();
        let mut curr_square = piece_data.curr_square.as_ref().unwrap().clone();
        loop {
            if let Some(new_square) = curr_square.new_with_offset(file_offset, rank_offset) {
                match board.get_piece_at_square(&new_square) {
                    Some(piece_at_square) if piece_at_square.white != piece_data.white => {
                        squares.push(new_square);
                        break;
                    },
                    None => {
                        curr_square = new_square.clone();
                        squares.push(new_square);
                    },
                    _ => break
                }
            }
        }

        squares
    }

    fn get_valid_squares(&self, piece_data: &PieceData, board: &Board) -> Vec<Square> {
        let mut squares = self.get_diagonal_squares(piece_data, board);
        squares.extend(self.get_straight_squares(piece_data, board));
        let move_only_squares: Vec<Square> = self.get_move_only_squares(piece_data).into_iter()
            .filter(|square| board.get_piece_at_square(square).is_none()).collect();

        let capture_only_squares: Vec<Square> = self.get_capture_only_squares(piece_data).into_iter()
            .filter(|square| {
                if let Some(piece) = board.get_piece_at_square(square) {
                    return piece.white != piece_data.white;
                }

                return false;
            }).collect();

        squares.extend(move_only_squares.into_iter());
        squares.extend(capture_only_squares.into_iter());

        squares
    }
}

struct PawnRules;

impl PawnRules {
    pub fn new() -> Self {
        Self
    }
}

impl PieceRules for PawnRules {
    fn get_initial_square(&self, piece_data: &PieceData) -> Square {
        let rank = if piece_data.white {Rank::R2} else {Rank::R7};
        let file = match piece_data.piece {
            UniquePiece::APawn => File::A,
            UniquePiece::BPawn => File::B,
            UniquePiece::CPawn => File::C,
            UniquePiece::DPawn => File::D,
            UniquePiece::EPawn => File::E,
            UniquePiece::FPawn => File::F,
            UniquePiece::GPawn => File::G,
            UniquePiece::HPawn => File::H,
            _ => panic!("Invalid UniquePiece for pawn initial square")
        };
        Square::new_known(file, rank)
    }

    fn get_move_only_squares(&self, piece_data: &PieceData) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        let direction = if piece_data.white {1} else {-1};
        let curr_square = piece_data.curr_square.as_ref().unwrap();
        if !piece_data.has_moved {
            let double_step_square = curr_square.new_with_offset(0, 2 * direction).unwrap();
            squares.push(double_step_square);
        }

        // Always unwrap because there can't be any un-promoted pawns in the last rank
        let single_step_square = curr_square.new_with_offset(0, direction).unwrap();
        squares.push(single_step_square);

        squares
    }

    fn get_capture_only_squares(&self, piece_data: &PieceData) -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        let direction = if piece_data.white {1} else {-1};
        let curr_square = piece_data.curr_square.as_ref().unwrap();

        if let Some(right_capture_square) = curr_square.new_with_offset(1, direction) {
            squares.push(right_capture_square);
        }

        if let Some(left_capture_square) = curr_square.new_with_offset(-1, direction) {
            squares.push(left_capture_square);
        }

        squares
    }
}

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
