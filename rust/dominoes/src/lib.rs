type Domino = (usize, usize);

pub fn chain(input: &Vec<Domino>) -> Option<Vec<Domino>> {
    if input.is_empty() {
        return Some(vec![])
    }

    let mut current_chain: Vec<Domino> = vec![input[0].to_owned()];
    let available_tiles = input[1..].iter().cloned().collect::<Vec<_>>();
    if dps(&mut current_chain, available_tiles) {
        Some(current_chain)
    } else {
        None
    }
}

fn dps(current_chain: &mut Vec<Domino>, available_tiles: Vec<Domino>) -> bool {
    let tail: usize = current_chain.last().unwrap().1;
    if available_tiles.is_empty() {
        return current_chain[0].0 == tail
    }

    for i in 0..(available_tiles.len()) {
        let tile = available_tiles[i];
        if tile.0 != tail && tile.1 != tail {
            continue
        }

        if tile.0 == tail {
            current_chain.push(tile.to_owned());
        } else if tile.1 == tail {
            current_chain.push((tile.1, tile.0));
        }

        let mut rest_tiles = available_tiles.clone();
        rest_tiles.remove(i);
        if dps(current_chain, rest_tiles) {
            return true
        }
        current_chain.pop();
    }
    false
}
