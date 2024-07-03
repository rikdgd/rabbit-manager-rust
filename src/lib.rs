mod pattern_builders;
mod pattern_managers;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::add;
    #[test]
    fn it_works() {
        assert_eq!(4, add(2, 2));
    }
}
