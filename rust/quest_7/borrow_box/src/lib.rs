#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nbr_of_games: u16
}

impl Game {
    pub fn new(i: u32, pl1: String, pl2: String, n: u16) -> Box<Game> {
        Box::new(Game{id: i, p1: (pl1, 0), p2: (pl2, 0), nbr_of_games: n})
    }

    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 > self.p2.1 {
            return self.p1.clone();
        } else if self.p1.1 < self.p2.1 {
            return self.p2.clone();
        } else {
            return ("Same score! tied".to_string(), self.p1.1);
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        if self.nbr_of_games != 0 {        
            if self.p1.0 == user_name {
                self.p1.1 += 1;
            }else {
                self.p2.1 += 1;
            }
            self.nbr_of_games -= 1;
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}