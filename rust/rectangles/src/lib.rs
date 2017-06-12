type Position = (usize, usize);

pub fn count(lines: &Vec<&str>) -> usize {
    if lines.is_empty() {
        return 0
    }

    let width = lines[0].len();
    let height = lines.len();
    let tiles = lines.iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut sum = 0;

    for r in 0..height {
        for c in 0..width {
            sum += match tiles[r][c] {
                '+' => count_at(&tiles, (c,r)),
                _ => 0
            }
        }
    }

    sum
}

fn count_at(tiles: &Vec<Vec<char>>, pos: Position) -> usize {
    let edges_east = get_edges(tiles, pos, (1,0));
    let edges_south = get_edges(tiles, pos, (0,1));

    let mut sum = 0;
    for e0 in &edges_east {
        for e1 in &edges_south {
            if get_edges(tiles, (e0.0,e0.1), (0,1)).contains(&(e0.0,e1.1)) 
                && get_edges(tiles, (e1.0,e1.1), (1,0)).contains(&(e0.0,e1.1)) {
                sum += 1
            }
        }
    }

    sum
}

fn get_edges(tiles: &Vec<Vec<char>>, pos: Position, dir: Position) -> Vec<Position> {
    let mut edges: Vec<Position> = vec![];
    let mut pos = pos.clone();
    loop {
        pos.0 += dir.0;
        pos.1 += dir.1;
        if pos.0 >= tiles[0].len() || pos.1 >= tiles.len() {
            return edges
        }

        match tiles[pos.1][pos.0] {
            '+' => edges.push(pos.clone()),
            '-' if dir == (1,0) => continue,
            '|' if dir == (0,1) => continue,
            _ => return edges
        }
    }
}
