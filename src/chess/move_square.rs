use std::fmt::format;


#[derive(PartialEq, Copy, Clone, Debug)] // for checking object equallity used in .contains of Vec
pub struct Move {
    initial: Square,
    end: Square,
    promotion: Option<Promotion>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Promotion {
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Square {
    pub rank: i32, // included in (1..=8)
    pub file: char,
}


impl Move {
    pub fn from_uci(coords: &str) -> Option<Self> {
        let starting = Square::from_str(&coords[0..2])?;
        let ending = Square::from_str(&coords[2..4])?;

        Some(Move {
            initial: starting,
            end: ending,
            promotion: match coords.chars().nth(4) { // Check if the move in uci has an extra char for promotion info
                Some(p) => {
                    match p {
                        'q' => Some(Promotion::Queen),
                        'r' => Some(Promotion::Rook),
                        'b' => Some(Promotion::Bishop),
                        'n' => Some(Promotion::Knight),
                        _ => {
                            println!("PAAANNNICCCC");
                            panic!("no other char is valid for promotion notation");
                        }
                    }
                },
                None => None,
            }
        })
    }

    pub fn from_squares(start: Square, end: Square, promotion: Option<Promotion>) -> Self {
        Move { initial: start, end: end, promotion: promotion } // no promotion by default when generating all moves
    }

    pub fn starting_square(&self) -> Square {
        self.initial
    }

    pub fn final_square(&self) -> Square {
        self.end   
    }

    pub fn to_uci(&self) -> String {
        match self.promotion {
            None => { format!("{}{}", self.initial.to_uci(), self.final_square().to_uci()) }
            Some(promotion) => {
                let mut move_string = format!("{}{}", self.initial.to_uci(), self.final_square().to_uci());
                match promotion {
                    Promotion::Queen => { move_string = format!("{}q", move_string); }
                    Promotion::Rook => { move_string = format!("{}r", move_string); }
                    Promotion::Knight => { move_string = format!("{}n", move_string); }
                    Promotion::Bishop => { move_string = format!("{}b", move_string); }
                }

                move_string
            }
        }
        
    }
}

const BASE_TEN: u32 = 10;

impl Square {
    // returns None if the position is outside the board
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

    // returns None if the position is outside the board
    pub fn offset(&self, file_offset: i8, rank_offset: i32) -> Option<Square> {
        let new_file = (self.file as i8 + file_offset) as u8 as char;
        let new_rank = self.rank as i32 + rank_offset;

        Square::new(new_file, new_rank as i32)
    }

    fn to_uci(&self) -> String {
        format!("{}{}", self.file, self.rank)
    }
}