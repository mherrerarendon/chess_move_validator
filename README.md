# chess_move_validator
A chess move validator written in Rust

[![Build status](https://img.shields.io/github/workflow/status/mherrerarendon/chess_move_validator/Rust)](https://github.com/mherrerarendon/chess_move_validator)
[![codecov](https://img.shields.io/codecov/c/github/mherrerarendon/chess_move_validator)](https://codecov.io/gh/mherrerarendon/chess_move_validator)
<br/>
## Features
- Setup chess board and get valid moves for piece:
```rust
use chess_move_validator::{UniquePiece};

let mut board = chess_move_validator::Board::new();

// Setup board
board.add_pgn_moves("1. e4 d5")?; 

let legal_moves = board.legal_moves_from_square(&Square::E4);
assert_eq!(legal_moves.len(), 2);

Ok(())
```
More to come...