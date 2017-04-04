pub struct Bowling{}

trait Bowls {
    fn roll(&self, p: usize) {}
    fn score(&self)->usize {0}
}
impl Bowls for Bowling {
}

impl Bowling {
    fn new() -> Bowling { Bowling{}}
}
#[test]
fn new_one() {
    let bowl = Bowling::new();
}
trait RollMany : Bowls {
    fn roll_many(self, p:usize, rolls:usize) -> Self;
}
impl RollMany for Bowling {
    fn roll_many(self, p:usize, rolls:usize) -> Bowling {
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
