
pub struct Move {
    starting_square: Square,
    end_square: Square,
}

pub struct Square {
    file: char,
    rank: u8,
}


impl Move {
    pub fn from_uci_coords(coords: &str) -> Option<Self> {
        let starting = Square::from_str(&coords[0..2])?;
        let ending = Square::from_str(&coords[2..4])?;

        Some(Move {
            starting_square: starting,
            end_square: ending,
        })
    }
}

const BASE_TEN: u32 = 10;

impl Square {
    pub fn new(file: char, rank: u8) -> Option<Self> {
        if ('a'..='h').contains(&file) && (1..=8).contains(&rank) {
            Some(Square { file: file, rank: rank })
        } else {
            None
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        let mut chars= s.chars();
        let file = chars.next()?;
        let rank = chars.next()?.to_digit(BASE_TEN).map(|d| d as u8)?;

        if ('a'..='h').contains(&file) && (1..=8).contains(&rank) {
            Some(Square { file: file, rank: rank })
        } else {
            None
        }
    }
}