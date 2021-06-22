extern crate mut_static;

extern "C" {
    fn notify_piecemoved(from_x: i32, from_y: i32, to_x: i32, to_y: i32);
    fn notify_piececrowned(x_coord: i32, y_coord: i32);
}

#[macro_use]
extern crate lazy_static;

use board::{Coordinate, GamePiece, Move, PieceColor};
use game::GameEngine;
use mut_static::MutStatic;

lazy_static! {
    pub static ref GAME_ENGINE: MutStatic<GameEngine> = MutStatic::from(GameEngine::new());
}

#[no_mangle]
pub extern "C" fn move_piece(from_x: i32, fy: i32, to_x: i32, ty: i32) -> i32 {
    let mut engine = GAME_ENGINE.write().unwrap();
    let move_made = Move::new((from_x as usize, fy as usize), (to_x as usize, ty as usize));
    let res = engine.move_piece(&move_made);
    match res {
        Ok(result_move) => {
            unsafe {
                notify_piecemoved(from_x, fy, to_x, ty);
            }
            if result_move.crowned {
                unsafe {
                    notify_piececrowned(to_x, ty);
                }
            }
            1
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn get_piece(x: i32, y: i32) -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    let piece_from_board = engine.get_piece(Coordinate(x as usize, y as usize));
    match piece_from_board {
        Ok(Some(piece)) => piece.into(),
        Ok(None) => -1,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn get_current_turn() -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    GamePiece::new(engine.current_turn()).into()
}

const PIECEFLAG_BLACK: u8 = 1;
const PIECEFLAG_WHITE: u8 = 2;
const PIECEFLAG_CROWN: u8 = 4;

impl Into<i32> for GamePiece {
    fn into(self) -> i32 {
        let mut val: u8 = 0;
        if self.color == PieceColor::Black {
            val += PIECEFLAG_BLACK;
        } else if self.color == PieceColor::White {
            val += PIECEFLAG_WHITE;
        }

        if self.crowned {
            val += PIECEFLAG_CROWN;
        }

        val as i32
    }
}

mod board;
mod game;
