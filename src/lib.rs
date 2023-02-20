#![no_std]

pub use hs::HashSet;

pub fn new() -> HashSet<usize> {
    HashSet::new()
}

pub fn insert(a: &mut HashSet<usize>, v: usize) {
    a.insert(v);
}

pub fn contains(a: &HashSet<usize>, v: usize) -> bool {
    a.contains(&v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut h = new();
        insert(&mut h, 0);
        assert!(contains(&h, 0));
        assert!(!contains(&h, 1));
    }
}
