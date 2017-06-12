use std::collections::HashMap;
use std::thread;
use std::sync::Arc;

pub fn frequency(texts: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // TODO: get away with not copying the input
    let texts: Arc<Vec<_>> = Arc::new(texts.iter().map(|s| s.to_string()).collect::<_>());
    let mut children = vec![];

    for i in 0..worker_count {
        let texts = texts.clone();

        children.push(thread::spawn(move || {
            let mut j: usize = i;
            let mut thread_map = HashMap::new();

            while j < texts.len() {
                for c in texts[j].chars() {
                    if c.is_alphabetic() {
                        *thread_map.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
                    }
                }
                j += worker_count;
            }

            thread_map
        }));
    }

    let mut map: HashMap<char, usize> = HashMap::new();
    for t in children {
        let thread_map = t.join().unwrap();
        for (c, n) in thread_map.iter() {
            *map.entry(*c).or_insert(0) += *n;
        }
    }

    map
}
