use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};

pub fn frequency(texts: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let map: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    // TODO: get away with not copying the input
    let texts: Arc<Vec<_>> = Arc::new(texts.iter().map(|s| s.to_string()).collect::<_>());
    let mut children = vec![];

    for i in 0..worker_count {
        let map = map.clone();
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

            let mut map = map.lock().unwrap();
            for (c, n) in thread_map.iter() {
                *map.entry(*c).or_insert(0) += *n;
            }
        }));
    }

    for t in children {
        t.join().unwrap();
    }

    // TODO: extract underlying hashmap instead of cloning it
    // Arc::try_unwrap(map).unwrap().into_inner().unwrap()
    let rs = map.lock().unwrap().clone();
    rs
}
