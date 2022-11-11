use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub};
use std::cmp::max;
pub struct BigNumber(Vec<u8>);
impl BigNumber {
    pub fn from(input: &str) -> Self {
        let x: Vec<u8> = input.chars().map(|c| {
            c.to_digit(10).unwrap() as u8
        }).rev().collect();
        Self(x)
    }
    pub fn make_literal_u128(&self) -> Option<u128> {
        // Returns the actual unsigned integer if possible.
        let data = &self.0;
        let mut sum = 0;
        for i in 0..data.len() {
            let power = 10_u128.checked_pow(i as u32)?;
            sum += data[i] as u128 * power;
        }

        Some(sum)
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.0.clone()
    }
}

// Implement some traits on this thing

impl Display for BigNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let x = self.0.iter()
            .rev()
            .map(|n| char::from_digit(*n as u32, 10).unwrap())
            .collect::<String>();
        write!(f, "{}", &x)
    }
}

impl Add for BigNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let max_i = max(self.0.len(), other.0.len());

        // out is the vector that should hold the sum of numbers
        let mut out:Vec<u8> = Vec::new();
        let mut carry = false;
        for i in 0..max_i {
            // Depending on the size of either, the vars below may be None.
            let n1 = self.0.get(i);
            let n2 = other.0.get(i);

            match (n1, n2) {
                (Some(a), Some(b)) => {
                    // Do actual math here
                    if carry {
                        out.push((*a + *b + 1) % 10);

                        // Check if carry is needed
                        if (*a + *b + 1) > 9 {
                            carry = true;
                        } else {
                            carry = false;
                        }
                    } else {
                        out.push((*a + *b) % 10);

                        // Check if carry is needed
                        if (*a + *b) > 9 {
                            carry = true;
                        } else {
                            carry = false;
                        }
                    }
                    // Check for end case
                    if i == max_i - 1 && carry {
                        out.push(1);
                    }
                },

                (Some(n), None) | (None, Some(n)) => {
                    if carry {
                        out.push((n + 1) % 10);

                        // Check again for carry
                        if n + 1 > 9 {
                            carry = true;
                        } else {
                            carry = false;
                        }
                    } else {
                        out.push(*n);
                    }
                    // Check for end case
                    if i == max_i - 1 && carry {
                        out.push(1);
                    }
                },
                _ => continue,
            }
        }
        // At this stage, should have a new vector "out" that contains the digits of the added thing
        BigNumber(out)
    }
}