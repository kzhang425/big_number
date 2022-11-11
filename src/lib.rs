// Import related packages here
mod big_number;

// Tests
#[cfg(test)]
mod tests {
    use crate::big_number::{BigNumber};

    #[test]
    fn literal() {
        let case = BigNumber::from("1234");
        assert_eq!(case.get_data(), vec![4, 3, 2, 1]);
    }

    #[test]
    fn display_trait() {
        let case = BigNumber::from("1234");
        assert_eq!(format!("{}", case), "1234");
    }

    #[test]
    fn test_add() {
        let case1 = BigNumber::from("12349");
        let case2 = BigNumber::from("2345");
        assert_eq!(format!("{}", case1 + case2), "14694");
    }

}


