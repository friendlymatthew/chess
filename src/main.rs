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
        let chess_move = Move::from(chess_move);

        if chess_move.is_take {
            todo!("how do we take? ")
        }

        // pawn push

        // we have th

        todo!()
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
