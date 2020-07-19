struct Board {
  black: u32,
  white: u32,
  kings: u32
}

impl Board {
    fn print(&self) {
        println!("  a   b   c   d   e   f   g   h  ");
        println!("  -------------------------------");
        let mut cursor: u32 = 1;
        for i in 1..9 {
            print!("{}|", 9-i);
            for j in 1..9 {
                if j%2 == 0 && i%2!=0 {
                    print!("___|");
                    continue;
                } else if j%2!=0 && i%2==0 {
                    print!("___|");
                    continue;
                }

                if cursor & self.black == cursor {
                    print!("_*_|");
                } else if cursor &  self.white == cursor {
                    print!("_0_|");
                } else {
                    print!("___|");
                }
                cursor<<=1;
            }
            print!("\n");
        }
        println!("\n*-black\n0-white");
    }

    fn move_(from: &str, to: &str) {

    }

    fn decode(pos: &str)->u8 {
        //decode position to index for bitboard
        0
    }
}


fn main() {
    let board = Board {
        white: 0b11111111_11110000_00000000_00000000,
        black: 0b00000000_00000000_00001111_11111111,
        kings: 0
    };
    board.print();
}
