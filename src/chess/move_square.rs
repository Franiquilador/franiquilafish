
#[derive(PartialEq, Copy, Clone, Debug)] // for checking object equallity used in .contains of Vec
pub struct Move {
    initial: Square,
    end: Square,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Square {
    pub rank: i32, // included in (1..=8)
    pub file: char,
}


impl Move {
    pub fn from_uci_coords(coords: &str) -> Option<Self> {
        let starting = Square::from_str(&coords[0..2])?;
        let ending = Square::from_str(&coords[2..4])?;

        Some(Move {
            initial: starting,
            end: ending,
        })
    }

    pub fn from_squares(start: Square, end: Square) -> Self {
        Move { initial: start, end: end }
    }

    pub fn get_starting_square(&self) -> Square {
        self.initial
    }

    pub fn get_final_square(&self) -> Square {
        self.end   
    }
}

const BASE_TEN: u32 = 10;

impl Square {
    pub fn new(file: char, rank: i32) -> Option<Self> {
        if ('a'..='h').contains(&file) && (1..=8).contains(&rank) {
            Some(Square { file: file, rank: rank })
        } else {
            None
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        let mut chars= s.chars();
        let file = chars.next()?; // ? means if its None, the function returns early. otherwise return the value inside option
        let rank = chars.next()?.to_digit(BASE_TEN).map(|d| d as i32)?;

        if ('a'..='h').contains(&file) && (1..=8).contains(&rank) {
            Some(Square { file: file, rank: rank })
        } else {
            None
        }
    }

    pub fn offset(&self, file_offset: i8, rank_offset: i32) -> Option<Square> {
        let new_file = (self.file as i8 + file_offset) as u8 as char;
        let new_rank = self.rank as i32 + rank_offset;

        Square::new(new_file, new_rank as i32)
    }
}