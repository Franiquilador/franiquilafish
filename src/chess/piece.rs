pub enum Piece {
    King,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub enum ChessPiece {
    Black(Piece),
    White(Piece),
}

/*
struct ChessPiece {
    color: Color,
    piece: Piece,
}



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
*/
