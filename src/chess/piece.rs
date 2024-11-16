pub enum Piece {
    King(Color),
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    Empty,
}

pub enum Color {
    Black,
    White,
}