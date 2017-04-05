pub struct Bowling{
    score: usize,
}

trait Bowls {
    fn roll(&mut self, p: usize);
    fn score(&self)->usize {0}
}
impl Bowls for Bowling {
    fn roll(&mut self, p: usize) {self.score += p;}
    fn score(&self)->usize {self.score}
}

impl Bowling {
    fn new() -> Bowling {
        Bowling{
            score:0,
        }
    }
}
#[test]
fn new_one() {
    let _ = Bowling::new();
}
trait RollMany : Bowls {
    fn roll_many(self, p:usize, rolls:usize) -> Self;
}
impl RollMany for Bowling {
    fn roll_many(mut self, p:usize, rolls:usize) -> Bowling {
        match rolls {
            0 => self,
            a => { self.roll(p); self.roll_many(p,a-1) }
        }
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
