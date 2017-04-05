pub struct Bowling{
    pin: Vec<usize>,
}

trait Bowls {
    fn roll(&mut self, p: usize);
    fn score(&self)->usize {0}
}
struct BowlingScore {
    score:usize,
    skip:bool,
    frame:usize,
    pins:Vec<usize>,
}
impl BowlingScore {
    fn new(p:&Vec<usize>) -> BowlingScore {
        let mut ps = p.clone();
        ps.push(0);
        ps.push(0);

        BowlingScore {
            score:0,
            skip:false,
            frame:0,
            pins:ps,
        }
    }
    fn no_skip(&mut self) {
        self.skip=false;
    }
    fn next_frame(&mut self) {
        self.frame += 1;
        self.skip = true;
    }
    fn done(&self) -> bool {
        self.frame > 10
    }
}
impl Bowls for Bowling {
    fn roll(&mut self, p: usize) {
        self.pin.push(p);
    }
    fn score(&self)->usize {
        let mut score = BowlingScore::new(&self.pin);

        for sl in score.pins.clone().windows(3) {
            if score.skip {
                score.no_skip();
                continue;
            }
            score.next_frame();

            if score.done() { return score.score }

            score.score += 
                match (sl[0], sl[1], sl[2]) {
                 (10, bo,nus) => {
                         score.no_skip();
                         10+ bo+nus
                     }
                 (sp,are, bonus) if sp+are==10 => 10+bonus,
                 (a,b,_) => a+b,
                };
        }
        score.score
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
