pub struct ChessPosition {
    r: i8,
    c: i8
}

impl ChessPosition {
    pub fn new(r: i8, c: i8) -> Result<ChessPosition, &'static str> {
        if r > -1 && r < 8 && c > -1 && c < 8 {
            Ok(ChessPosition { r: r, c: c })
        } else {
            Err("Bad position")
        }
    }
}

pub struct Queen {
    position: ChessPosition
}

impl Queen {
    pub fn new(p: ChessPosition) -> Queen {
        Queen { position: p }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.r == other.position.r
            || self.position.c == other.position.c
            || self.position.r - other.position.r == self.position.c - other.position.c
            || other.position.r - self.position.r == self.position.c - other.position.c
    }
}
