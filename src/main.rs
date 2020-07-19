struct Board{
  black: u64,
  white: u64
}

impl Board {
    fn print(&self) {
        println!("  a   b   c   d   e   f   g   h  ");
        println!("  -------------------------------");
        let mut cursor: u64 = 1;
        for i in 1..9 {
            print!("{}|", 9-i);
            for _ in 0..8 { 
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
}


fn main() {
    let board = Board{
        white:0b01010101_10101010_01010101_00000000_00000000_00000000_00000000_00000000,
        black:0b00000000_00000000_00000000_00000000_00000000_10101010_01010101_10101010
    };
    board.print();
}
