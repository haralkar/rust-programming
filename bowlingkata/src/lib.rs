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
        let mut frame = 0;

        let mut to_score = self.pin.clone();
        to_score.push(0);
        to_score.push(0);
        for sl in to_score.windows(3) {
            if skip {
                skip=false;
                continue;
            }
            skip=true;
            frame += 1;
            if frame > 10 { break ; }

            score += 
                match (sl[0], sl[1], sl[2]) {
                 (10,b1,b2) => {
                         skip=false;
                         10+b1+b2
                     }
                 (sp,are,bonus) if sp+are==10 => 10+bonus,
                 (a,b,_) => a+b,
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
    fn strike(self) -> Self;
}
impl RollMany for Bowling {
    fn roll_many(mut self, p:usize, rolls:usize) -> Bowling {
        match rolls {
            0 => self,
            a => { self.roll(p); self.roll_many(p,a-1) }
        }
    }
    fn strike(mut self) -> Self {
        self.roll(10);
        self
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
#[test]
fn spare() {
    let bowl = Bowling::new().spare().roll_many(1, 18);
    assert_eq!(bowl.score(), 29);
}
#[test]
fn strike() {
    let bowl = Bowling::new().strike().roll_many(1, 18);
    assert_eq!(bowl.score(), 30);
}
#[test]
fn perfect_game() {
    let bowl = Bowling::new().roll_many(10, 12);
    assert_eq!(bowl.score(), 300);
}

//*
// */
