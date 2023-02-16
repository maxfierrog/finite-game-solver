// Max Fierro, maxfierro@berkeley.edu
// Wednesday, February 15th, 2023


use super::Outcome;


#[derive(Clone)]
pub struct Board {
    contents: Vec<Vec<Option<bool>>>,
    height: i32,
    width: i32,
    win: i32
}


impl Board {
    pub fn new(height: i32, width: i32, win: i32) -> Self {
        let mut contents = Vec::new();
        for _ in 0..height {
            let row: Vec<Option<bool>> = vec![None; width as usize];
            contents.push(row);
        }
        Board {
            contents,
            height,
            width,
            win
        }
    }

    pub fn symbol_at(&self, i: i32, j: i32) -> Option<bool> {
        if i >= self.height || i < 0 || j >= self.width || j < 0 {
            panic!("Out of bounds read on board.");
        }
        self.contents[i as usize][j as usize]
    }

    pub fn place(&mut self, what: Option<bool>, i: i32, j:i32) {
        if i >= self.height || i < 0 || j >= self.width || j < 0 {
            panic!("Out of bounds write to board.");
        }
        self.contents[i as usize][j as usize] = what;
    }

    pub fn transform(&mut self, flip: i32, rotate: i32) {
        for _ in 0..flip {
            self.flip();
        }
        for _ in 0..rotate {
            self.rotate();
        }
    }

    pub fn outcome(&self) -> Option<Outcome> {
        let moves_max = self.width * self.height;
        let mut moves_made = 0;
        for i in self.contents.iter() {
            for j in i {
                if let Some(_) = j {
                    moves_made += 1;
                }
            }
        }
        if self.horizontal_win()
            || self.vertical_win() 
            || self.diagonal_win() {
            Some(Outcome::Loss)
        } else if moves_made == moves_max {
            Some(Outcome::Tie)
        } else {
            None
        }
    }

    pub fn hash(&self) -> i32 {
        let mut hash = 0;
        let mut counter = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if let Some(curr) = self.symbol_at(i, j) {
                    if curr {
                        hash += 3_i32.pow(counter);
                    } else {
                        hash += 2 * 3_i32.pow(counter);
                    }
                }
                counter += 1;
            }
        }
        hash
    }

    pub fn canonical(&self) -> Board {
        let mut max_hash = 0;
        let mut canon = self.clone();
        if self.height == self.width {
            // All symmetries in dihedral group of ord. 8
            for f in 0..2 {
                for r in 0..4 {
                    let mut new_board = self.clone();
                    new_board.transform(f, r);
                    new_board.print();
                    if new_board.hash() > max_hash {
                        canon = new_board;
                        max_hash = canon.hash();
                    }
                }
            }
        } else {
            // Keep board the same, and reflect it vertically
            for f in 0..2 {
                let mut new_board = self.clone();
                new_board.transform(f, 0);
                if new_board.hash() > max_hash {
                    canon = new_board;
                    max_hash = canon.hash();
                }
            }

            // Reflect board horizontally
            let mut new_board = self.clone();
            new_board.transform(0, 1);
            new_board.transform(1, 3);
            if new_board.hash() > max_hash {
                canon = new_board;
                max_hash = canon.hash();
            }

            // Rotate board 180 degrees
            let mut new_board = self.clone();
            new_board.transform(0, 2);
            if new_board.hash() > max_hash {
                canon = new_board;
            }
        }
        canon
    }

    pub fn print(&self) {
        for i in self.contents.iter() {
            for j in i {
                if let Some(c) = j {
                    if *c {
                        print!("X ");
                    } else {
                        print!("O ")
                    }
                } else {
                    print!("  ");
                }
            }
            print!("\n");
        }
    }

    /* HELPER METHODS */

    // Reflects board along its horizontal axis
    fn flip(&mut self) {
        let mut new_contents = Vec::new();
        for _ in 0..self.height {
            let row: Vec<Option<bool>> = vec![None; self.width as usize];
            new_contents.push(row);
        }
        for i in 0..self.height {
            new_contents[(self.height - i - 1) as usize] 
                = self.contents[i as usize].clone();
        }
        self.contents = new_contents;
    }

    // Rotates board 90 degrees to the right
    fn rotate(&mut self) {
        let mut new_contents = Vec::new();
        for _ in 0..self.width {
            let row: Vec<Option<bool>> = vec![None; self.height as usize];
            new_contents.push(row);
        }
        for i in 0..self.height {
            for j in 0..self.width {   
                new_contents[j as usize][(self.height - i - 1) as usize] 
                    = self.symbol_at(i, j);
            }
        }
        let temp = self.width;
        self.width = self.height;
        self.height = temp;
        self.contents = new_contents;
    }

    fn vertical_win(&self) -> bool {
        let mut count: i32;
        let mut kind: Option<bool>;
        let mut curr: Option<bool>;
        let mut win: bool = false;
        for j in 0..self.width {
            curr = self.symbol_at(0, j);
            kind = curr;
            count = 1;
            for i in 1..self.height {
                curr = self.symbol_at(i, j);
                if curr == kind && curr != None {
                    count += 1;
                } else {
                    count = 1;
                    kind = curr;
                }
                if count == self.win {
                    win = true;
                    break;
                }
            }
            if win {
                break;
            }
        }
        win
    }

    fn horizontal_win(&self) -> bool {
        let mut count: i32;
        let mut kind: Option<bool>;
        let mut curr: Option<bool>;
        let mut win: bool = false;
        for i in 0..self.height {
            curr = self.symbol_at(i, 0);
            kind = curr;
            count = 1;
            for j in 1..self.width {
                curr = self.symbol_at(i, j);
                if curr == kind && curr != None {
                    count += 1;
                } else {
                    count = 1;
                    kind = curr;
                }
                if count == self.win {
                    win = true;
                    break;
                }
            }
        }
        win
    }

    fn diagonal_win(&self) -> bool {
        let mut win = false;
        for i in 0..(self.width - self.win + 1) {
            for j in 0..(self.height - self.win + 1) {
                if self.check_diag_win_from(i, j) {
                    win = true;
                }
            }
        }
        // this is so dumb 
        let mut b = self.clone();
        b.flip();
        for i in 0..(b.width - b.win + 1) {
            for j in 0..(b.height - b.win + 1) {
                if b.check_diag_win_from(i, j) {
                    win = true;
                }
            }
        }
        win
    }

    fn check_diag_win_from(&self, i: i32, j: i32) -> bool {
        if let Some(first)= self.symbol_at(i, j) {
            for k in 1..self.win {
                if let None = self.symbol_at(i+k, j+k) {
                    return false;
                }
                if let Some(kind) = self.symbol_at(i+k, j+k) {
                    if first != kind {
                        return false;
                    }
                }
            }
        } else {
            return false;
        }
        true
    }
}