const RANK: u64 = 8;

struct Board {
    white: u64,
    black: u64,

    white_turn: bool,
}

struct Move {
    is_take: bool,
    //          e  4 = 20
    start_position: u64,
    end_position: u64,
}

impl<'a> From<&'a str> for Move {
    fn from(value: &'a str) -> Self {
        // how do we parse into a move
        todo!()
    }
}

impl Board {
    fn make_move(&mut self, chess_move: &str) {
        let Move { is_take, start_position, end_position } = Move::from(chess_move);

        if is_take {
            if self.white_turn {

            }
        } else {

            if self.white_turn {
                if self.is_valid(self.white, start_position) {
                    self.white ^= start_position | end_position;
                }
            } else {
                if self.is_valid(self.black, start_position) {
                    let mask = start_position | end_position;
                    self.black ^= mask;
                }
            }
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

        todo!()
    }
    fn is_valid(&self, side: u64, position: u64) -> bool {
       (side & position) > 0
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white: 0b1111_1111 << RANK,
            black: 0b1111_1111 << (RANK * 6),
            white_turn: true,
        }
    }
}

fn print_bitboard(bitboard: u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{char} ");
        }
        println!();
    }
}

fn main() {
    let b = Board::default();

    print_bitboard(b.white | b.black);
}
