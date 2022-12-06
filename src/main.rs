use std::io;

use rand::Rng;

enum BaccaratResult {
    Win,
    Lose,
    Draw,
}

struct MonteCarlo {
    seq: Vec<i32>,
    is_next_bet: bool
}

impl MonteCarlo {
    fn win(&mut self) {
        self.seq.remove(0);
        self.seq.pop();
        self.is_next_bet = self.seq.len() > 1;
    }
    
    fn lose(&mut self) {
        self.seq.push(self.seq.first().unwrap() + self.seq.last().unwrap());
        self.is_next_bet = self.seq.len() > 1;
    }

    fn next_bet_number(&self) -> Option<i32> {
        if self.is_next_bet != true {
            None
        } else {
            Some(self.seq.first().unwrap() + self.seq.last().unwrap())
        }
    }

    fn is_next_bet(&self) -> bool {
        self.is_next_bet
    }
}

fn play_baccarat() -> BaccaratResult {
    // Win 45%
    // Lose 45%
    // Draw 10%
    let num = rand::thread_rng().gen_range(0..100) + 1;
    if num <= 45 {
        BaccaratResult::Win
    } else if num >= 56 {
        BaccaratResult::Lose
    } else {
        BaccaratResult::Draw
    }
}

fn main() -> io::Result<()> {
    let num_of_play = 1000;
    let mut total = 0;
    let mut max_bet = 0;
    let mut win = 0;
    let mut lose = 0;
    let mut draw = 0;

    for _ in 0..num_of_play {
        let seq = vec![1,2,3];
        let is_next_bet = seq.len() > 1;
        let mut monte_carlo = MonteCarlo{
            seq: seq,
            is_next_bet: is_next_bet,
        };

        while monte_carlo.is_next_bet() {
            let next_bet = monte_carlo.next_bet_number().unwrap();
            total -= next_bet;
            if total < max_bet {
                max_bet = total
            }
            let ret = play_baccarat();
            match ret {
                BaccaratResult::Win => {
                    monte_carlo.win();
                    total += next_bet * 2;
                    win += 1;
                },
                BaccaratResult::Lose => {
                    monte_carlo.lose();
                    lose += 1;
                },
                BaccaratResult::Draw => {
                    total += next_bet;
                    draw += 1;
                },
            }
        }
    }

    println!("Total return: {}", total);
    println!("Max bet: {}", max_bet);
    println!("Win: {}", win);
    println!("Lose: {}", lose);
    println!("Draw: {}", draw);
    Ok(())
}
