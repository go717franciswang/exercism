pub fn primes_up_to(n: usize) -> Vec<usize> {
    let mut l: Vec<bool> = vec![false; n+1];
    for i in 2..(n+1) {
        if !l[i] {
            for j in i..(n/i+1) {
                l[i*j] = true;
            }
        }
    }

    l.iter()
        .enumerate()
        .filter(|&(_i,b)| !b)
        .map(|(i,_b)| i)
        .skip(2) // skips 0 and 1 which are not primes
        .collect::<Vec<usize>>()
}
