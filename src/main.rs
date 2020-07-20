use std::io;

struct Board {
  light: u32,
  dark: u32,
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

                if cursor & self.dark == cursor {
                    print!("_*_|");
                } else if cursor &  self.light == cursor {
                    print!("_0_|");
                } else {
                    print!("___|");
                }
                cursor<<=1;
            }
            print!("\n");
        }
        println!("\n*-dark\n0-light");
    }

    fn move_(from: &str, to: &str) {

    }
   
    fn decode(pos: &str)->u8 {
        //decode position to index for bitboard
        0
    }
}


fn read_input()->String {
    let mut line = String::new();
    io::stdin().
        read_line(&mut line).
        expect("Can't read line");
    line.trim().to_string()
}


fn main() {
    let board = Board {
        light: 0b11111111_11110000_00000000_00000000,
        dark:  0b00000000_00000000_00001111_11111111,
        kings: 0
    };
    loop {
        board.print();
        println!("Enter move or q for quit:");
        let input = read_input();
        if input == "q" {
            break;
        }
        //
    }

    
}
