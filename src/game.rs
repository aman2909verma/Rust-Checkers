use super::board::{Coordinate, GamePiece, Move, PieceColor};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
}

pub struct MoveResult {
    pub move_made: Move,
    pub crowned: bool,
}

impl GameEngine {
    pub fn new() -> GameEngine {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
        };
        engine.initialize_pieces();
        engine
    }

    pub fn initialize_pieces(&mut self) {
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
            .map(|(x_coord, y_coord)| (*x_coord as usize, *y_coord as usize))
            .for_each(|(x_coord, y_coord)| {
                self.board[x_coord][y_coord] = Some(GamePiece::new(PieceColor::White));
            });

        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .map(|(x_coord, y_coord)| (*x_coord as usize, *y_coord as usize))
            .for_each(|(x_coord, y_coord)| {
                self.board[x_coord][y_coord] = Some(GamePiece::new(PieceColor::Black));
            });
    }

    pub fn move_piece(&mut self, move_desired: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.legal_moves();

        if !legal_moves.contains(move_desired) {
            return Err(());
        }

        let Coordinate(from_x, from_y) = move_desired.from;
        let Coordinate(to_x, to_y) = move_desired.to;
        let piece = self.board[from_x][from_y].unwrap();
        let midpiece_coordinate = self.midpiece_coordinate(from_x, from_y, to_x, to_y);
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            self.board[x][y] = None; // remove the jumped piece
        }

        // Move piece from source to dest
        self.board[to_x][to_y] = Some(piece);
        self.board[from_x][from_y] = None;

        let crowned = if self.should_crown(piece, move_desired.to) {
            self.crown_piece(move_desired.to);
            true
        } else {
            false
        };
        self.advance_turn();

        Ok(MoveResult {
            move_made: move_desired.clone(),
            crowned,
        })
    }

    pub fn get_piece(&self, coord: Coordinate) -> Result<Option<GamePiece>, ()> {
        let Coordinate(x, y) = coord;
        if x <= 7 && y <= 7 {
            Ok(self.board[x][y])
        } else {
            Err(())
        }
    }

    pub fn current_turn(&self) -> PieceColor {
        self.current_turn
    }

    fn advance_turn(&mut self) {
        if self.current_turn == PieceColor::Black {
            self.current_turn = PieceColor::White
        } else {
            self.current_turn = PieceColor::Black
        }
        self.move_count += 1;
    }

    // Black pieces in row 0 or White pieces in row 7 are crowned
    fn should_crown(&self, piece: GamePiece, coord: Coordinate) -> bool {
        let Coordinate(_x, y) = coord;

        (y == 0 && piece.color == PieceColor::Black) || (y == 7 && piece.color == PieceColor::White)
    }

    fn crown_piece(&mut self, coord: Coordinate) -> bool {
        let Coordinate(x, y) = coord;
        if let Some(piece) = self.board[x][y] {
            self.board[x][y] = Some(GamePiece::crowned(piece));
            true
        } else {
            false
        }
    }

    pub fn is_crowned(&self, coord: Coordinate) -> bool {
        let Coordinate(x, y) = coord;
        match self.board[x][y] {
            Some(piece) => piece.crowned,
            None => false,
        }
    }

    pub fn move_count(&self) -> u32 {
        self.move_count
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut vmoves = self.valid_moves_from(loc);
                        moves.append(&mut vmoves);
                    }
                }
            }
        }

        moves
    }

    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        if let Some(p) = self.board[x][y] {
            let mut jumps = loc
                .jump_targets_from()
                .filter(|t| self.valid_jump(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                })
                .collect::<Vec<Move>>();
            let mut moves = loc
                .move_targets_from()
                .filter(|t| self.valid_move(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                })
                .collect::<Vec<Move>>();
            jumps.append(&mut moves);
            jumps
        } else {
            Vec::new()
        }
    }

    fn midpiece_coordinate(
        &self,
        x: usize,
        y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Option<Coordinate> {
        if to_x == x + 2 && to_y == y + 2 {
            Some(Coordinate(x + 1, y + 1))
        } else if x >= 2 && y >= 2 && to_x == x - 2 && to_y == y - 2 {
            Some(Coordinate(x - 1, y - 1))
        } else if x >= 2 && to_x == x - 2 && to_y == y + 2 {
            Some(Coordinate(x - 1, y + 1))
        } else if y >= 2 && to_x == x + 2 && to_y == y - 2 {
            Some(Coordinate(x + 1, y - 1))
        } else {
            None
        }
    }

    fn midpiece(&self, x: usize, y: usize, to_x: usize, to_y: usize) -> Option<GamePiece> {
        match self.midpiece_coordinate(x, y, to_x, to_y) {
            Some(Coordinate(x, y)) => self.board[x][y],
            None => None,
        }
    }

    fn valid_jump(&self, moving_piece: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        if !to.on_board() || !from.on_board() {
            false
        } else {
            let Coordinate(from_x, from_y) = *from;
            let Coordinate(to_x, to_y) = *to;

            let midpiece = self.midpiece(from_x, from_y, to_x, to_y);
            match midpiece {
                Some(piece) if piece.color != moving_piece.color => true,
                _ => false,
            }
        }
    }

    fn valid_move(&self, moving_piece: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        if !to.on_board() || !from.on_board() {
            false
        } else {
            let Coordinate(to_x, to_y) = *to;
            if let Some(_piece) = self.board[to_x][to_y] {
                false
            } else {
                let Coordinate(_from_x, from_y) = *from;
                let mut valid = false;
                if to_y > from_y && moving_piece.color == PieceColor::White {
                    // white moves down
                    valid = true;
                }
                if to_y < from_y && moving_piece.color == PieceColor::Black {
                    // black moves up
                    valid = true;
                }
                if to_y > from_y && moving_piece.color == PieceColor::Black && moving_piece.crowned
                {
                    // crowned black move down
                    valid = true;
                }
                if to_y < from_y && moving_piece.color == PieceColor::White && moving_piece.crowned
                {
                    // crowned white move up
                    valid = true;
                }
                valid
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::board::{Coordinate, GamePiece, Move, PieceColor};
    use super::GameEngine;

    #[test]
    fn should_crown() {
        let engine = GameEngine::new();
        let black = GamePiece::new(PieceColor::Black);
        let res = engine.should_crown(black, Coordinate(3, 0));
        assert!(res);
        let res_no_crown = engine.should_crown(black, Coordinate(5, 2));
        assert_eq!(res_no_crown, false);
    }

    #[test]
    fn mut_crown() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let crowned = engine.crown_piece(Coordinate(1, 0));
        assert!(crowned);
        assert!(engine.is_crowned(Coordinate(1, 0)));
    }

    #[test]
    fn advance_turn() {
        let mut engine = GameEngine::new();
        engine.advance_turn();
        assert_eq!(engine.current_turn(), PieceColor::White);
        engine.advance_turn();
        assert_eq!(engine.current_turn(), PieceColor::Black);
        assert_eq!(engine.move_count(), 2);
    }

    #[test]
    fn move_targets() {
        let coord_1 = Coordinate(0, 5);
        let targets = coord_1.move_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(targets, [Coordinate(1, 6), Coordinate(1, 4)]);

        let coord_2 = Coordinate(1, 6);
        let targets2 = coord_2.move_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(
            targets2,
            [
                Coordinate(0, 7),
                Coordinate(2, 7),
                Coordinate(2, 5),
                Coordinate(0, 5)
            ]
        );

        let coord_3 = Coordinate(2, 5);
        let targets3 = coord_3.move_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(
            targets3,
            [
                Coordinate(1, 6),
                Coordinate(3, 6),
                Coordinate(3, 4),
                Coordinate(1, 4)
            ]
        );
    }

    #[test]
    fn valid_from() {
        let coord_1 = Coordinate(0, 5);
        let coord_2 = Coordinate(2, 5);

        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let move_1 = engine.valid_moves_from(coord_1);
        let move_2 = engine.valid_moves_from(coord_2);
        assert_eq!(
            move_1,
            [Move {
                from: Coordinate(0, 5),
                to: Coordinate(1, 4),
            }]
        );
        assert_eq!(
            move_2,
            [
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(3, 4),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(1, 4),
                }
            ]
        );
    }

    #[test]
    fn legal_moves_black() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let moves = engine.legal_moves();
        assert_eq!(
            moves,
            [
                Move {
                    from: Coordinate(0, 5),
                    to: Coordinate(1, 4),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(3, 4),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(1, 4),
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(5, 4),
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(3, 4),
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(7, 4),
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(5, 4),
                }
            ]
        );
    }

    #[test]
    fn legal_moves_white() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        engine.advance_turn();
        let moves = engine.legal_moves();
        assert_eq!(
            moves,
            [
                Move {
                    from: Coordinate(1, 2),
                    to: Coordinate(0, 3),
                },
                Move {
                    from: Coordinate(1, 2),
                    to: Coordinate(2, 3),
                },
                Move {
                    from: Coordinate(3, 2),
                    to: Coordinate(2, 3),
                },
                Move {
                    from: Coordinate(3, 2),
                    to: Coordinate(4, 3),
                },
                Move {
                    from: Coordinate(5, 2),
                    to: Coordinate(4, 3),
                },
                Move {
                    from: Coordinate(5, 2),
                    to: Coordinate(6, 3),
                },
                Move {
                    from: Coordinate(7, 2),
                    to: Coordinate(6, 3),
                }
            ]
        );
    }

    #[test]
    fn jump_targets() {
        let coord_1 = Coordinate(3, 3);
        let targets = coord_1.jump_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(
            targets,
            [
                Coordinate(5, 1),
                Coordinate(5, 5),
                Coordinate(1, 1),
                Coordinate(1, 5)
            ]
        );
    }

    #[test]
    fn jump_moves_validation() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        engine.board[1][4] = Some(GamePiece::new(PieceColor::White)); // this should be jumpable from 0,5 to 2,3
        let moves = engine.legal_moves();
        assert_eq!(
            moves,
            [
                Move {
                    from: Coordinate(0, 5),
                    to: Coordinate(2, 3),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(0, 3)
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(3, 4)
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(5, 4)
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(3, 4)
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(7, 4)
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(5, 4)
                }
            ]
        );
    }

    #[test]
    fn test_basic_move() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let res = engine.move_piece(&Move::new((0, 5), (1, 4)));
        assert!(res.is_ok());

        let old = engine.board[0][5];
        let new = engine.board[1][4];
        assert_eq!(old, None);
        assert_eq!(
            new,
            Some(GamePiece {
                color: PieceColor::Black,
                crowned: false
            })
        );

        // fail to perform illegal move
        let res = engine.move_piece(&Move::new((1, 4), (2, 4))); // can't move horiz
        assert!(!res.is_ok());
        assert_eq!(engine.board[2][4], None);
    }
}
