pub struct BigNumber(Vec<u8>);
impl BigNumber {
    pub fn from(input: &str) -> Self {
        let x: Vec<u8> = input.chars().map(|c| {
            c.to_digit(10).unwrap() as u8
        }).rev().collect();
        Self(x)
    }
    pub fn make_literal(&self) -> Option<u128> {
        // Returns the actual unsigned integer if possible.
        let data = &self.0;
        let mut sum = 0;
        for i in 0..data.len() {
            let power = 10_u128.checked_pow(i as u32)?;
            sum += data[i] as u128 * power;
        }

        Some(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {
        let case = BigNumber::from("1234");
        assert_eq!(case.0, vec![4, 3, 2, 1]);
    }

}
