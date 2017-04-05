pub struct Bowling{
    pin: Vec<usize>,
}

trait Bowls {
    fn roll(&mut self, p: usize);
    fn score(&self)->usize {0}
}
impl Bowls for Bowling {
    fn roll(&mut self, p: usize) {
        self.pin.push(p);
    }
    fn score(&self)->usize {
        let mut score = 0;
        let mut skip = false;
        let mut to_score = self.pin.clone();
        to_score.push(0);
        to_score.push(0);
        for sl in to_score.windows(3) {
            if skip {
                skip=false;
                continue;
            }
            skip=true;
            score += 
                match sl[0] + sl[1] {
                 10 => 10 + sl[2],
                  a => a,
                };
        }
        score
    }
}

impl Bowling {
    fn new() -> Bowling {
        Bowling{
            pin: Vec::with_capacity(22),
        }
    }
}
#[test]
fn new_one() {
    let _ = Bowling::new();
}
trait RollMany : Bowls {
    fn roll_many(self, p:usize, rolls:usize) -> Self;
    fn spare(self) -> Self;
}
impl RollMany for Bowling {
    fn roll_many(mut self, p:usize, rolls:usize) -> Bowling {
        match rolls {
            0 => self,
            a => { self.roll(p); self.roll_many(p,a-1) }
        }
    }
    fn spare(mut self) -> Self {
        self.roll(4);
        self.roll(6);
        self
    }
}

#[test]
fn gutter_game() {
    let bowl = Bowling::new().roll_many(0, 20);
    assert_eq!(bowl.score(), 0);
}
#[test]
fn all_ones() {
    let bowl = Bowling::new().roll_many(1, 20);
    assert_eq!(bowl.score(), 20);
}
//*
#[test]
fn spare() {
    let bowl = Bowling::new().spare().roll_many(1, 18);
    assert_eq!(bowl.score(), 29);
}
// */
