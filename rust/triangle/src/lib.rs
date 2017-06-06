pub struct Triangle {
    sides: [u32; 3]
}

impl Triangle {
    pub fn build(sides: [u32; 3]) -> Result<Triangle, &'static str> {
        if sides.iter().any(|s| *s == 0) {
            return Err("Invalid input")
        }

        let cc: f64 = sides.iter().map(|x| *x as f64).sum::<f64>()/2_f64;
        if sides.iter().any(|s| (*s as f64) > cc) {
            return Err("Invalid input")
        }

        Ok(Triangle { sides: sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] 
            && self.sides[0] == self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_scalene()
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[0] != self.sides[2]
            && self.sides[1] != self.sides[2]
    }
}
