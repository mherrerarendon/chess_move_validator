use std::{collections::HashMap, fmt::Display};

pub use chess_pgn_parser::{
    parse_move_sequence, peggler::ParseError, File, Move::BasicMove, Move::CastleKingside,
    Move::CastleQueenside, Piece, Rank, Square,
};

mod piece_data;
mod position_cursor;
mod rules;

use piece_data::PieceData;
use position_cursor::PositionCursor;
pub use rules::UniquePiece;

use std::convert::From;
use std::fmt;

use crate::rules::{BishopRules, KnightRules, QueenRules, RookRules};

type Position = HashMap<(UniquePiece, bool), Square>;

pub struct Board {
    pieces: Vec<PieceData>,
}

#[derive(Debug)]
pub enum ChessError {
    IlegalMove(UniquePiece, Square, Square),
    PgnParseError,
}

impl From<ParseError> for ChessError {
    fn from(_item: ParseError) -> Self {
        ChessError::PgnParseError
    }
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IlegalMove(piece, from, to) => {
                write!(
                    f,
                    "Invalid move. Piece: {:?}, from: {:?}, to: {:?}",
                    piece, from, to
                )
            }
            Self::PgnParseError => write!(f, "Failed to parse pgn string"),
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: Self::create_initial_pieces(),
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

    fn position_count(&self) -> usize {
        self.pieces
            .first()
            .expect("board can't be empty")
            .square_hist
            .len()
    }

    pub fn position_cursor(&self) -> PositionCursor {
        PositionCursor::new(self)
    }

    fn disambiguate_from_square(
        &self,
        piece: Piece,
        white: bool,
        from: &Square,
        to: &Square,
    ) -> Square {
        let piece_data_list = self.get_all_live_piece_data_with_type(piece, white);
        if let Some(rank) = from.rank() {
            if let Some(piece_data) = piece_data_list
                .iter()
                .find(|p| p.curr_square().unwrap().rank().unwrap() == rank)
            {
                return piece_data.curr_square().unwrap().clone();
            }
        } else if let Some(file) = from.file() {
            if let Some(piece_data) = piece_data_list
                .iter()
                .find(|p| p.curr_square().unwrap().file().unwrap() == file)
            {
                return piece_data.curr_square().unwrap().clone();
            }
        } else {
            for piece_data in piece_data_list {
                let valid_squares = piece_data.behavior.get_valid_squares(piece_data, &self);
                if valid_squares.iter().find(|s| *s == to).is_some() {
                    return piece_data.curr_square().unwrap().clone();
                }
            }
        }
        panic!("Unable to disambiguate from square");
    }

    fn add_basic_move(
        &mut self,
        piece: Piece,
        white: bool,
        to: &Square,
        from: &Square,
        is_capture: bool,
        promoted_to: Option<Piece>,
    ) {
        if let Some(captured_piece_data) = self.get_mut_piece_data_at_square(to) {
            assert!(is_capture);
            captured_piece_data.capture();
        }
        let known_from = match from.get_known() {
            Some(known_from) => known_from,
            None => self.disambiguate_from_square(piece, white, from, to),
        };
        let piece_data = self
            .get_mut_piece_data_at_square(&known_from)
            .expect("Missing piece");
        piece_data.move_unchecked(to.clone());
        if let Some(promotion) = promoted_to {
            match promotion {
                chess_pgn_parser::Piece::Rook => piece_data.behavior = Box::new(RookRules),
                chess_pgn_parser::Piece::Knight => piece_data.behavior = Box::new(KnightRules),
                chess_pgn_parser::Piece::Bishop => piece_data.behavior = Box::new(BishopRules),
                chess_pgn_parser::Piece::Queen => piece_data.behavior = Box::new(QueenRules),
                _ => panic!("Invalid promotion"),
            }
        }
    }

    fn add_castle_move(
        &mut self,
        rank: Rank,
        old_rook_file: File,
        new_king_file: File,
        new_rook_file: File,
    ) {
        let old_king_square = Square::new_known(File::E, rank);
        let new_king_square = Square::new_known(new_king_file, rank);
        let old_rook_square = Square::new_known(old_rook_file, rank);
        let new_rook_square = Square::new_known(new_rook_file, rank);
        let king_piece_data = self
            .get_mut_piece_data_at_square(&old_king_square)
            .expect("Missing piece");
        king_piece_data.move_unchecked(new_king_square);

        let rook_data = self
            .get_mut_piece_data_at_square(&old_rook_square)
            .expect("Missing piece");
        rook_data.move_unchecked(new_rook_square);
    }

    pub fn add_pgn_moves(&mut self, pgn_moves: &str) -> Result<(), ParseError> {
        let game_moves = parse_move_sequence(pgn_moves)?;
        for game_move in game_moves.moves.iter() {
            match game_move.move_.move_ {
                BasicMove {
                    piece,
                    ref to,
                    ref from,
                    is_capture,
                    promoted_to,
                } => {
                    self.add_basic_move(
                        piece,
                        game_move.number.is_some(),
                        to,
                        from,
                        is_capture,
                        promoted_to,
                    );
                }
                ref c @ CastleKingside | ref c @ CastleQueenside => {
                    let rank = if game_move.number.is_some() {
                        Rank::R8
                    } else {
                        Rank::R1
                    };
                    let (old_rook_file, new_king_file, new_rook_file) = match c {
                        CastleKingside => (File::H, File::G, File::F),
                        CastleQueenside => (File::A, File::C, File::D),
                        _ => unreachable!(),
                    };
                    self.add_castle_move(rank, old_rook_file, new_king_file, new_rook_file);
                }
            }
        }
        Ok(())
    }

    pub fn legal_moves_from_square(&self, square: &Square) -> Vec<Square> {
        let piece_data = self
            .get_piece_data_at_square(square)
            .expect("missing piece");
        piece_data.behavior.get_valid_squares(piece_data, self)
    }

    pub fn get_valid_squares_for_piece(&self, piece: UniquePiece, white: bool) -> Vec<Square> {
        let piece_data = self
            .pieces
            .iter()
            .find(|p| p.piece == piece && p.white == white)
            .expect("missing piece");
        if piece_data.curr_square().is_some() {
            piece_data.behavior.get_valid_squares(piece_data, &self)
        } else {
            Vec::new()
        }
    }

    pub fn get_piece_data_at_square(&self, square: &Square) -> Option<&PieceData> {
        for piece in self.pieces.iter() {
            if let Some(curr_square) = piece.curr_square() {
                if curr_square == square {
                    return Some(piece);
                }
            }
        }
        return None;
    }

    pub fn get_mut_piece_data_at_square(&mut self, square: &Square) -> Option<&mut PieceData> {
        for piece in self.pieces.iter_mut() {
            if let Some(curr_square) = piece.curr_square() {
                if curr_square == square {
                    return Some(piece);
                }
            }
        }
        return None;
    }

    fn get_all_live_piece_data_with_type(&self, piece: Piece, white: bool) -> Vec<&PieceData> {
        self.pieces
            .iter()
            .filter(|p| {
                piece == Self::unique_to_piece(p.piece)
                    && p.curr_square().is_some()
                    && p.white == white
            })
            .collect()
    }

    fn unique_to_piece(unique_piece: UniquePiece) -> Piece {
        match unique_piece {
            UniquePiece::QRook | UniquePiece::KRook => Piece::Rook,
            UniquePiece::QKnight | UniquePiece::KKnight => Piece::Knight,
            UniquePiece::QBishop | UniquePiece::KBishop => Piece::Bishop,
            UniquePiece::Queen => Piece::Queen,
            UniquePiece::King => Piece::King,
            _ => Piece::Pawn,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::UniquePiece;

    use super::Board;
    use chess_pgn_parser::{peggler::ParseError, Square};
    use std::collections::HashSet;

    fn assert_valid_squares(expected: &[Square], actual: &[Square]) {
        let expected_set: HashSet<_> = expected.iter().collect();
        let actual_set: HashSet<_> = actual.iter().collect();
        assert_eq!(expected_set, actual_set);
    }

    #[test]
    fn test_pawn_capture() -> Result<(), ParseError> {
        let mut board = Board::new();
        board.add_pgn_moves("1. d4 e5")?;
        let pawn = board
            .get_piece_data_at_square(&Square::D4)
            .expect("missing piece.");
        assert_eq!(pawn.piece, UniquePiece::DPawn);
        let valid_squares = pawn.behavior.get_valid_squares(&pawn, &board);
        assert_valid_squares(&[Square::E5, Square::D5], &valid_squares);

        let pawn = board
            .get_piece_data_at_square(&Square::E5)
            .expect("missing piece.");
        assert_eq!(pawn.piece, UniquePiece::EPawn);
        let valid_squares = pawn.behavior.get_valid_squares(&pawn, &board);
        assert_valid_squares(&[Square::D4, Square::E4], &valid_squares);
        Ok(())
    }

    #[test]
    fn test_pawn_en_passant() -> Result<(), ParseError> {
        // TODO
        Ok(())
    }

    #[test]
    fn test_rook_capture() -> Result<(), ParseError> {
        let mut board = Board::new();

        // Remove pawns that are in the way of testing rook
        let mut pawn = board
            .get_mut_piece_data_at_square(&Square::A2)
            .expect("missing piece");
        pawn.capture();
        pawn = board
            .get_mut_piece_data_at_square(&Square::A7)
            .expect("missing piece.");
        pawn.capture();

        let rook = board
            .get_piece_data_at_square(&Square::A1)
            .expect("missing piece.");
        assert_eq!(rook.piece, UniquePiece::QRook);
        let valid_squares = rook.behavior.get_valid_squares(rook, &board);
        assert_valid_squares(
            &[
                Square::A2,
                Square::A3,
                Square::A4,
                Square::A5,
                Square::A6,
                Square::A7,
                Square::A8,
            ],
            &valid_squares,
        );

        let rook = board
            .get_piece_data_at_square(&Square::A8)
            .expect("missing piece.");
        assert_eq!(rook.piece, UniquePiece::QRook);
        let valid_squares = rook.behavior.get_valid_squares(rook, &board);
        assert_valid_squares(
            &[
                Square::A1,
                Square::A2,
                Square::A3,
                Square::A4,
                Square::A5,
                Square::A6,
                Square::A7,
            ],
            &valid_squares,
        );
        Ok(())
    }

    #[test]
    fn test_knight_capture() -> Result<(), ParseError> {
        let mut board = Board::new();
        board.add_pgn_moves("1. Nc3 Nf6 2. Ne4 a6")?;
        let knight = board
            .get_piece_data_at_square(&Square::F6)
            .expect("missing piece");
        assert_eq!(knight.piece, UniquePiece::KKnight);
        let valid_squares = knight.behavior.get_valid_squares(knight, &board);
        assert_valid_squares(
            &[Square::G8, Square::H5, Square::G4, Square::E4, Square::D5],
            &valid_squares,
        );

        let knight = board
            .get_piece_data_at_square(&Square::E4)
            .expect("missing piece");
        assert_eq!(knight.piece, UniquePiece::QKnight);
        let valid_squares = knight.behavior.get_valid_squares(knight, &board);
        assert_valid_squares(
            &[
                Square::F6,
                Square::G5,
                Square::G3,
                Square::C3,
                Square::C5,
                Square::D6,
            ],
            &valid_squares,
        );
        Ok(())
    }

    #[test]
    fn test_bishop_capture() -> Result<(), ParseError> {
        let mut board = Board::new();
        board.add_pgn_moves("1. b3 e6 2. Ba3 a6")?;
        let bishop = board
            .get_piece_data_at_square(&Square::A3)
            .expect("missing piece.");
        assert_eq!(bishop.piece, UniquePiece::QBishop);
        let valid_squares = bishop.behavior.get_valid_squares(bishop, &board);
        assert_valid_squares(
            &[
                Square::B4,
                Square::C5,
                Square::D6,
                Square::E7,
                Square::F8,
                Square::B2,
                Square::C1,
            ],
            &valid_squares,
        );

        let bishop = board
            .get_piece_data_at_square(&Square::F8)
            .expect("missing piece.");
        assert_eq!(bishop.piece, UniquePiece::KBishop);
        let valid_squares = bishop.behavior.get_valid_squares(bishop, &board);
        assert_valid_squares(
            &[Square::A3, Square::B4, Square::C5, Square::D6, Square::E7],
            &valid_squares,
        );
        Ok(())
    }

    #[test]
    fn test_queen_capture() -> Result<(), ParseError> {
        let mut board = Board::new();
        board.add_pgn_moves("1. c3 e6 2. Qa4 Qh4")?;
        let queen = board
            .get_piece_data_at_square(&Square::A4)
            .expect("missing piece.");
        assert_eq!(queen.piece, UniquePiece::Queen);
        let valid_squares = queen.behavior.get_valid_squares(queen, &board);
        assert_valid_squares(
            &[
                Square::A3,
                Square::B3,
                Square::C2,
                Square::D1,
                Square::A5,
                Square::A6,
                Square::A7,
                Square::B4,
                Square::C4,
                Square::D4,
                Square::E4,
                Square::F4,
                Square::G4,
                Square::H4,
                Square::B5,
                Square::C6,
                Square::D7,
            ],
            &valid_squares,
        );

        let queen = board
            .get_piece_data_at_square(&Square::H4)
            .expect("missing piece.");
        assert_eq!(queen.piece, UniquePiece::Queen);
        let valid_squares = queen.behavior.get_valid_squares(queen, &board);
        assert_valid_squares(
            &[
                Square::H3,
                Square::H2,
                Square::G3,
                Square::F2,
                Square::A4,
                Square::B4,
                Square::C4,
                Square::D4,
                Square::E4,
                Square::F4,
                Square::G4,
                Square::G5,
                Square::F6,
                Square::E7,
                Square::D8,
                Square::H5,
                Square::H6,
            ],
            &valid_squares,
        );
        Ok(())
    }

    #[test]
    fn test_promotion() -> Result<(), ParseError> {
        let mut board = Board::new();
        board.add_pgn_moves("1. c4 f5 2. c5 f4 3. c6 f3 4. cxb7 fxg2 5. bxc8=N gxf8=N")?;
        let promoted_knight = board
            .get_piece_data_at_square(&Square::C8)
            .expect("missing piece.");
        let valid_squares = promoted_knight
            .behavior
            .get_valid_squares(promoted_knight, &board);
        assert_valid_squares(
            &[Square::A7, Square::B6, Square::D6, Square::E7],
            &valid_squares,
        );
        Ok(())
    }
}
