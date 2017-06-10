pub fn annotate(board: &Vec<&str>) -> Vec<String> {
    let mut rs: Vec<String> = vec![];
    if board.is_empty() {
        return rs
    }

    let width = board[0].len();
    let height = board.len();
    for r in 0..height {
        rs.push(board[r].char_indices().map(|(c, cell)| {
            match cell {
                '*' => '*',
                ' ' => {
                    let n = count(board, r, c, width, height);
                    if n == 0 {
                        ' '
                    } else {
                        ('0' as u8 + n) as char
                    }
                },
                _ => panic!()
            }
        }).collect::<String>());
    }

    rs
}

fn count(board: &Vec<&str>, r0: usize, c0: usize, width: usize, height: usize) -> u8 {
    let mut n = 0_u8;
    for dr in -1..2 {
        for dc in -1..2 {
            if dr == 0 && dc == 0 {
                continue
            }

            let r1 = r0 as i8 + dr;
            let c1 = c0 as i8 + dc;
            if r1 < 0 || r1 >= height as i8 || c1 < 0 || c1 >= width as i8 {
                continue
            }

            if board[r1 as usize].chars().nth(c1 as usize).unwrap() == '*' {
                n += 1;
            }
        }
    }
    n
}
