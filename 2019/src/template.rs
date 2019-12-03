fn do_your_thing() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(do_your_thing(), 0)
    }

    fn data() -> Vec<isize> {
        let input: Vec<isize> = vec![];
        return input;
    }
}