#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}

impl GamePiece {
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
            crowned: false,
        }
    }

    pub fn crowned(p: GamePiece) -> GamePiece {
        GamePiece {
            color: p.color,
            crowned: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn on_board(self) -> bool {
        let Coordinate(x_coord, y_coord) = self;
        x_coord <= 7 && y_coord <= 7
    }

    pub fn jump_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut jumps = Vec::new();
        let Coordinate(x_coord, y_coord) = *self;
        if y_coord >= 2 {
            jumps.push(Coordinate(x_coord + 2, y_coord - 2));
        }
        jumps.push(Coordinate(x_coord + 2, y_coord + 2));

        if x_coord >= 2 && y_coord >= 2 {
            jumps.push(Coordinate(x_coord - 2, y_coord - 2));
        }
        if x_coord >= 2 {
            jumps.push(Coordinate(x_coord - 2, y_coord + 2));
        }
        jumps.into_iter()
    }

    pub fn move_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut moves = Vec::new();
        let Coordinate(x_coord, y_coord) = *self;
        if x_coord >= 1 {
            moves.push(Coordinate(x_coord - 1, y_coord + 1));
        }
        moves.push(Coordinate(x_coord + 1, y_coord + 1));
        if y_coord >= 1 {
            moves.push(Coordinate(x_coord + 1, y_coord - 1));
        }
        if x_coord >= 1 && y_coord >= 1 {
            moves.push(Coordinate(x_coord - 1, y_coord - 1));
        }
        moves.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Move {
        Move {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
}
