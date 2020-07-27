use std::io;
use std::collections::HashMap;

struct Board<'a> {
  light: u32,
  dark: u32,
  kings: u32,
  position_lookup_table: HashMap<&'a str, i32>
}

#[derive(Debug)]
struct Move<'a> {
    sub_moves: Vec<&'a i32>,
    is_attack: bool
}

//      Position Lookup Table
//    _a__b__c__d__e__f__g__h_
// 8 |28|__|29|__|30|__|31|__|
// 7 |__|24|__|25|__|26|__|27|
// 6 |20|__|21|__|22|__|23|__|
// 5 |__|16|__|17|__|18|__|19|
// 4 |12|__|13|__|14|__|15|__|
// 3 |__|08|__|09|__|10|__|11|
// 2 |04|__|05|__|06|__|07|__|
// 1 |__|00|__|01|__|02|__|03|
//

impl<'a> Board<'a> {
    fn new() -> Board<'a> {
        let position_lookup_table = [
            ("b1",  0), ("d1",  1), ("f1",  2), ("h1",  3),
            ("a2",  4), ("c2",  5), ("e2",  6), ("g2",  7),
            ("b3",  8), ("d3",  9), ("f3", 10), ("h3", 11),
            ("a4", 12), ("c4", 13), ("e4", 14), ("g4", 15),
            ("b5", 16), ("d5", 17), ("f5", 18), ("h5", 19),
            ("a6", 20), ("c6", 21), ("e6", 22), ("g6", 23),
            ("b7", 24), ("d7", 25), ("f7", 26), ("h7", 27),
            ("a8", 28), ("c8", 29), ("e8", 30), ("g8", 31)
        ].iter().cloned().collect();

        let light: u32 = 0b11111111_11110000_00000000_00000000;
        let dark: u32 = 0b00000000_00000000_00001111_11111111;
        let kings: u32 = 0b00000000_00000000_00000000_00000000;

        Board {light, dark, kings, position_lookup_table}

    }

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

    fn play(&self, move_: &Move) {
        //todo implement me
        println!{"move {:?}", move_}
    }

    fn decode(&self, pos: &str) -> &i32 {
        self.position_lookup_table.get(pos).unwrap()
    }

    fn parse_move(&self, input: &String) -> Move {
        let is_attack = input.contains(':');
        let path_str: Vec<&str> = input.split(|c| c == '-' || c == ':').collect();
        let sub_moves = path_str.iter().map(|c| self.decode(c)).collect();
        Move{sub_moves, is_attack}
    }
}


fn read_input() -> String {
    let mut line = String::new();
    io::stdin().
        read_line(&mut line).
        expect("Can't read line");
    line.trim().to_string()
}

fn main() {

    let board = Board::new();

    loop {
        board.print();
        println!("Enter move or q for quit:");
        let input = read_input();
        if input == "q" {
            break;
        }
        board.play(&board.parse_move(&input));
    }

}
