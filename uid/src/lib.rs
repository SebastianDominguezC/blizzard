#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use rand::{thread_rng, Rng};

pub struct Uid {}

impl Uid {
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
