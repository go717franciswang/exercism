pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rs: Vec<Vec<u32>> = vec![];
        if self.row_count == 0 {
            return rs
        }

        rs.push(vec![1_u32]);
        for i in 1..self.row_count {
            let mut row = vec![1_u32];
            let i = i as usize;
            let mut mid: Vec<u32> =
                rs[i-1][1..i].iter()
                .zip(rs[i-1][0..(i-1)].iter())
                .map(|(a,b)| a+b)
                .collect();
            row.append(&mut mid);
            row.push(1_u32);
            rs.push(row);
        }
        rs
    }
}
