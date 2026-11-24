use anyhow::{Result, bail, ensure};

use crate::bitboard::Bitboard;

mod bitboard;

const RANK: u64 = 8;

struct Board {
    white: Bitboard,
    black: Bitboard,

    white_turn: bool,
    last_move: Move,
}

impl Board {
    fn print(&self) {
        println!("\n   a b c d e f g h");

        for rank in (0..8).rev() {
            print!("{}  ", rank + 1);

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

            print!(" {}", rank + 1);
            println!();
        }

        println!("   a b c d e f g h");
    }
}

#[derive(Clone, Copy, PartialEq)]
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

impl Default for Move {
    fn default() -> Self {
        Move {
            is_take: false,
            start_position: Bitboard::from(0),
            end_position: Bitboard::from(0),
        }
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
    fn make_move(&mut self, chess_move: &str) -> Result<()> {
        let next_move = Move::from(chess_move);

        let Move {
            is_take,
            start_position,
            end_position,
        } = next_move;

        // validation
        {
            let start_rank = start_position.0.trailing_zeros() / 8;
            let end_rank = end_position.0.trailing_zeros() / 8;

            // e3 to e3 is invalid
            ensure!(start_position != end_position, "you must move");

            // you can't go backwards

            if self.white_turn {
                ensure!(start_rank < end_rank, "white can't go backwards");
            } else {
                ensure!(start_rank > end_rank, "black can't go backwards");
            }

            ensure!(
                (end_position & (self.black | self.white)).0 == 0,
                "can't go to an occupied position"
            );

            // you must be jumping from a valid square
            let side = if self.white_turn {
                self.white
            } else {
                self.black
            };

            ensure!(
                (side & start_position).0 > 0,
                "you must jump from a valid square"
            );
        }

        // we have our white pieces (wp)
        // we work backwards
        // if it's going to e4, it must be either in e3 or e2
        //
        // 1. we check if wp bits are set at e3 or e2 (wp & (e3 or e2))
        // if false => illegal move
        //
        // 2. update the position
        // wp & ((at start_position = 0) | (at end_position = 1))

        let (side, opp) = if self.white_turn {
            (&mut self.white, &mut self.black)
        } else {
            (&mut self.black, &mut self.white)
        };

        if is_take {
            if *side & start_position == 0 {
                bail!("invalid start position");
            }

            let (forward, enp) = if self.white_turn {
                (start_position << RANK, end_position >> RANK)
            } else {
                (start_position >> RANK, end_position << RANK)
            };
            let diag0 = forward << 1;
            let diag1 = forward >> 1;

            if end_position != diag0 && end_position != diag1 {
                bail!("invalid taking move");
            }

            // check that the opponent is there to be taken
            if *opp & end_position != 0 {
                // normal take, fine
                *side ^= start_position | end_position;
                *opp ^= end_position;
            } else if *opp & enp & self.last_move.end_position != 0 {
                // en passant take, fine
                *side ^= start_position | end_position;
                *opp ^= self.last_move.end_position;
            } else {
                bail!("invalid take position");
            }
        } else {
            let mask = start_position | end_position;
            *side ^= mask;
        }

        self.white_turn = !self.white_turn;
        self.last_move = next_move;

        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white: Bitboard::from(0b1111_1111) << RANK,
            black: Bitboard::from(0b1111_1111) << (RANK * 6),
            white_turn: true,
            last_move: Move::default(),
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
                if let Err(msg) = board.make_move(cmd) {
                    println!("invalid move! {:?}", msg);
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
    fn to_position_a8_is_top_left() {
        insta::assert_debug_snapshot!(Move::to_position("a8"), @"

        1 0 0 0 0 0 0 0
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

        assert!(b.make_move("e5 e6").is_err());
    }

    #[test]
    fn pawns_going_backwards_is_invalid() {
        let mut b = Board::default();

        assert!(b.make_move("e2 e1").is_err());
        assert!(b.white_turn);

        // valid move
        b.make_move("e2 e3").unwrap();
        assert!(b.make_move("e6 e7").is_err());
        assert!(!b.white_turn);
    }

    #[test]
    fn pawn_cannot_move_with_opp_pawn_in_front() {
        let mut board = Board {
            white: Move::to_position("e4"),
            black: Move::to_position("e5"),
            white_turn: true,
            last_move: Move::default(),
        };
        assert!(board.make_move("e4 e5").is_err());
        board.white_turn = false;
        assert!(board.make_move("e5 e4").is_err());
    }

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

    #[test]
    fn black_pawn_takes_from_d5_to_e4() {
        let mut board = Board {
            white: Move::to_position("e4"),
            black: Move::to_position("d5"),
            white_turn: false,
            last_move: Move::default(),
        };

        board.make_move("d5 take e4").unwrap();

        insta::assert_debug_snapshot!(board.black, @"

        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 1 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        ");

        insta::assert_debug_snapshot!(board.white, @"

        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0
        ");

        assert!(board.white_turn);
    }

    #[test]
    fn black_pawn_cannot_take_straight_forward() {
        let mut board = Board {
            white: Move::to_position("d4"),
            black: Move::to_position("d5"),
            white_turn: false,
            last_move: Move::default(),
        };

        assert!(board.make_move("d5 take d4").is_err());
        assert_eq!(board.black, Move::to_position("d5"));
        assert_eq!(board.white, Move::to_position("d4"));
        assert!(!board.white_turn);
    }
}
