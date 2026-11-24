use crate::bitboard::Bitboard;

mod bitboard;

const RANK: u64 = 8;

struct Board {
    white: Bitboard,
    black: Bitboard,

    white_turn: bool,
}

impl Board {
    fn print(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let mask = 1u64 << (rank * 8 + file);

                if self.white.0 & mask != 0 {
                    print!("\x1b[32m1\x1b[0m ");
                } else if self.black.0 & mask != 0 {
                    print!("\x1b[31m1\x1b[0m ");
                } else {
                    print!("0 ");
                }
            }
            println!();
        }
    }
}

struct Move {
    is_take: bool,
    //          e  4 = 20
    start_position: Bitboard,
    end_position: Bitboard,
}

impl Move {
    fn to_position(s: &str) -> Bitboard {
        let s_bytes = s.as_bytes();
        let col: u64 = (s_bytes[0] - b'a').into();
        let row: u64 = (s_bytes[1] - b'1').into();
        ((1 << col) << (row * RANK)).into()
    }
}

impl<'a> From<&'a str> for Move {
    fn from(value: &'a str) -> Self {
        // how do we parse into a move
        let mut parts = value.split_ascii_whitespace();
        let start = parts.next().unwrap();
        let mut end = parts.next().unwrap();
        let is_take = end == "take";
        if is_take {
            end = parts.next().unwrap();
        }

        Move {
            is_take,
            start_position: Move::to_position(start),
            end_position: Move::to_position(end),
        }
    }
}

impl Board {
    fn make_move(&mut self, chess_move: &str) -> Option<()> {
        let Move {
            is_take,
            start_position,
            end_position,
        } = Move::from(chess_move);

        if is_take {
            todo!("how do we take? ")
        }

        // pawn push

        // we have our white pieces (wp)
        // we work backwards
        // if it's going to e4, it must be either in e3 or e2
        //
        // 1. we check if wp bits are set at e3 or e2 (wp & (e3 or e2))
        // if false => illegal move
        //
        // 2. update the position
        // wp & ((at start_position = 0) | (at end_position = 1))

        if is_take {
            if self.white_turn {}
        } else {
            let side = if self.white_turn {
                &mut self.white
            } else {
                &mut self.black
            };

            let is_valid = Self::is_valid(*side, start_position);

            if !is_valid {
                return None;
            }

            let mask = start_position | end_position;
            *side ^= mask;
        }

        self.white_turn = !self.white_turn;

        // pass
        Some(())
    }

    fn is_valid(side: Bitboard, position: Bitboard) -> bool {
        (side & position).0 > 0
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white: Bitboard::from(0b1111_1111) << RANK,
            black: Bitboard::from(0b1111_1111) << (RANK * 6),
            white_turn: true,
        }
    }
}

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut board = Board::default();
    let mut input = String::new();

    board.print();

    loop {
        print!("> ");
        io::stdout().flush()?;

        input.clear();

        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        match input.trim() {
            "exit" | "quit" => break,
            "" => continue,
            cmd => {
                if board.make_move(cmd).is_none() {
                    println!("invalid move!")
                }

                board.print();
            }
        }

        println!();
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_position_a1_is_bottom_left() {
        insta::assert_debug_snapshot!(Move::to_position("a1"), @"

        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        1 0 0 0 0 0 0 0
        ");
    }

    #[test]
    fn to_position_a8_is_bottom_right() {
        insta::assert_debug_snapshot!(Move::to_position("h1"), @"

        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 1
        ");
    }

    #[test]
    fn to_position_h8_is_top_right() {
        insta::assert_debug_snapshot!(Move::to_position("h8"), @"

        0 0 0 0 0 0 0 1
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        ");
    }

    #[test]
    fn e5_to_e7_is_invalid() {
        let mut b = Board::default();

        assert!(b.make_move("e5 e6").is_none());
    }

    // todo: fix me
    //     #[test]
    //     fn pawns_going_backwards_is_invalid() {
    //         let mut b = Board::default();
    //
    //         assert!(b.make_move("e2 e1").is_none());
    //     }

    #[test]
    fn from_parses_simple_move() {
        let m = Move::from("e2 e4");
        assert!(!m.is_take);
        assert_eq!(Move::to_position("e2"), m.start_position);
        assert_eq!(m.end_position, Move::to_position("e4"));
    }

    #[test]
    fn from_parses_take_move() {
        let m = Move::from("e4 take d5");
        assert!(m.is_take);
        assert_eq!(m.start_position, Move::to_position("e4"));
        assert_eq!(m.end_position, Move::to_position("d5"));
    }
}
