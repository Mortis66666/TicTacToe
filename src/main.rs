use std::io;

struct Board {
    board: [[i8; 3]; 3]
}

impl Board {
    fn new() -> Self {
        Self {
            board: [[0; 3]; 3]
        }
    }
    fn place(&mut self, x: usize, y: usize, n: i8) -> Result<(), io::Error> {
        if self.board[y][x] != 0 {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "This place is already placed"));
        }
        self.board[y][x] = n;
        Ok(())
    }
    fn print(&mut self) {
        for (y, row) in self.board.iter().enumerate() {
            let mut str_row = String::new();
            for (x, n) in row.iter().enumerate() {
                if n == &1 {
                    str_row.push_str("X");
                } else if n == &-1{
                    str_row.push_str("O");
                } else {
                    str_row.push_str(&(y * 3 + x + 1).to_string()[..]);
                }
                if x < 2 {
                    str_row.push_str(" ");
                }
            }
            println!("{}", str_row);
        }
    }
    fn get_winner(&mut self) -> i8 {
        for row in self.board {
            let r: i8 = row.iter().sum();
            if r == -3 {
                return -1;
            } else if r == 3 {
                return 1;
            }
        }
        for x in 0..3 as usize {
            let mut r = 0;

            for row in self.board {
                r += row[x];
            }

            if r == -3 {
                return -1;
            } else if r == 3 {
                return 1;
            }
        }

        let mut r = 0;
        for i in 0..3 as usize {
            r += self.board[i][i];
        }
        if r == -3 {
            return -1;
        } else if r == 3 {
            return 1;
        }

        let mut r = 0;
        for i in 0..3 as usize {
            r += self.board[2-i][i];
        }
        if r == -3 {
            return -1;
        } else if r == 3 {
            return 1;
        }

        0
    }
}


fn main() {
    let mut board = Board::new();
    let mut someone_won = false;
    let mut turn = 0;

    while turn < 9 {
        board.print();
        let win_state = board.get_winner();
        if win_state == 1 {
            println!("X won!");
            someone_won = true;
            break;
        } else if win_state == -1 {
            println!("O won!");
            someone_won = true;
            break;
        } else {
            let (c, n) = if turn % 2 == 0 {('X', 1)} else {('O', -1)};
            println!("Player {}'s turn to play: ", c);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let place_id: usize = input.trim().parse().unwrap();

            if place_id > 9 {
                println!("Invalid place id, try again!");
                continue;
            }

            let x = (place_id - 1) % 3;
            let y = (place_id - 1) / 3;
            
            if let Err(err) = board.place(x, y, n) {
                println!("{}, try again!", err.into_inner().unwrap());
                continue;
            }
        }

        turn += 1;
    }

    if !someone_won {
        println!("Its a draw!");
    }
}
