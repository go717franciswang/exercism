pub struct BowlingGame {
    frames: Vec<Frame>
}

struct Frame {
    id: usize, // 1 to 10
    throws: Vec<u16>,
}

impl Frame {
    fn is_valid(&self, pins: u16) -> bool {
        if pins > 10 {
            return false
        }

        if self.throws.len() == 1 && self.throws[0] < 10 {
            return self.throws[0] + pins <= 10
        }

        if self.throws.len() == 2 && self.id < 9 {
            return false
        }

        if self.throws.len() == 2 && self.id == 9 {
            return match (self.throws[0], self.throws[1]) {
                (10, 10) => true,
                (10, n) => n + pins <= 10,
                (m, n) => m + n == 10,
            }
        }

        true
    }

    fn is_over(&self) -> bool {
        match self.throws.len() {
            0 => false,
            1 => self.id < 9 && self.throws[0] == 10,
            2 => self.id < 9 || self.throws[0]+self.throws[1] < 10,
            _ => true
        }
    }

    fn push(&mut self, pins: u16) {
        self.throws.push(pins);
    }

    fn is_strike(&self) -> bool {
        self.throws[0] == 10
    }

    fn is_spare(&self) -> bool {
        self.throws[0] + self.throws[1] == 10
    }

    fn total(&self) -> u16 {
        self.throws.iter().sum()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { frames: vec![] }
    }

    fn get_frame(&mut self) -> Option<&mut Frame> {
        if self.frames.is_empty() || self.frames.last().unwrap().is_over() {
            let id = self.frames.len();
            if id > 9 {
                return None
            }

            self.frames.push(Frame { id: id, throws: vec![] });
        }

        self.frames.last_mut()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), &'static str> {
        match self.get_frame() {
            Some(mut frame) => {
                if frame.is_valid(pins) {
                    Ok(frame.throws.push(pins))
                } else {
                    Err("Bad roll")
                }
            },
            None => Err("Game over")
        }
    }

    pub fn score(&self) -> Result<u16, &'static str> {
        if self.frames.len() < 10 || !self.frames[9].is_over() {
            return Err("Game not over")
        }

        let mut s = 0;
        for i in 0..(self.frames.len()-1) {
            let f = &self.frames[i];
            s += f.total();
            if f.is_strike() {
                s += self.strike_bonus(f);
            } else if f.is_spare() {
                s += self.frames[i+1].throws[0];
            }
        }

        Ok(s + self.frames[9].total())
    }

    fn strike_bonus(&self, frame: &Frame) -> u16 {
        let next_frame = &self.frames[frame.id+1];
        if next_frame.throws.len() >= 2 {
            next_frame.throws[0] + next_frame.throws[1]
        } else {
            next_frame.throws[0] + self.frames[frame.id+2].throws[0]
        }
    }
}

