use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn mode(v: &Vec<i32>) -> i32 {
    let mut modmap = HashMap::new();
    for i in v {
        // Check out the unintuitive API for HashMap Entry (second example)
        // https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
        // *map.entry("poneyland").or_insert(10) *= 2;

        let count = modmap.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;
    for (key, value) in modmap {
        // if value > max {
        //     max = value;
        //     mode = *key;
        // }

        // match value.cmp(&max) {
        //     Ordering::Greater => {
        //         max = value;
        //         mode = *key;
        //     }
        //     _ => {}
        // }
        if let Ordering::Greater = value.cmp(&max) {
            max = value;
            mode = *key;
        }
    }
    return mode;
}

pub fn create_random_vector(size: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    for _ in 0..size {
        let entry = rng.gen_range(0..size);
        v.push(entry);
    }
    return v;
}

pub fn median(v: &Vec<i32>) -> i32 {
    let mut v = v.clone();
    v.sort();
    let len = v.len();
    if len % 2 == 0 {
        let mid = len / 2;
        return (v[mid] + v[mid - 1]) / 2;
    } else {
        return v[len / 2];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_random_vector() {
        let v = create_random_vector(10);
        assert_eq!(v.len(), 10);
    }

    #[test]
    fn test_median() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(median(&v), 3);
    }

    #[test]
    fn test_mode() {
        let v = vec![1, 2, 3, 4, 5, 5, 3, 5, 5];
        assert_eq!(mode(&v), 5);
    }
}
