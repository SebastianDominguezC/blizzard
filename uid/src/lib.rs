//! # UID crate
//! This crate is utility crate to create unique identifications.
//!
//! Crate originally developed for the Blizzard Game and Server engines.

#[cfg(test)]
mod tests {
    use crate::Uid;
    #[test]
    fn test() {
        let id = Uid::new_numerical(2);
        println!("{}", id);
        assert!(id < 100);
        assert!(id > 0);
    }
}

use rand::{thread_rng, Rng};

/// UID struct that has different UID implementations.
pub struct Uid {}

impl Uid {
    /// Make a numerical uid with size = length
    pub fn new_numerical(length: u32) -> u32 {
        let mut id = 0;
        for i in 0..length {
            let mut rng = thread_rng();

            // Exclusive range
            let int: u32 = rng.gen_range(0..10);
            let base = u32::pow(10, i);
            id += base * int;
        }
        id
    }
}
