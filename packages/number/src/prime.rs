use num_integer::Roots;

pub trait IsPrime {
    fn is_prime(&self) -> bool;
}

macro_rules! impl_is_prime_trait {
    ($($t:tt)*) => {
        $(
            impl IsPrime for $t {
                #[inline]
                fn is_prime(&self) -> bool {
                    if *self <= 1 {
                       return false;
                    }

                    if *self < 4 {
                       return true;
                    }

                    if *self % 2 == 0 {
                       return false;
                    }

                    if *self < 9 {
                       return true;
                    }

                    if self % 3 == 0 {
                       return false;
                    }

                    let limit = self.sqrt();
                    let mut number = 5;

                    while number <= limit {
                        if self % number == 0 {
                            return false;
                        }

                        if self % (number + 2) == 0 {
                            return false;
                        }

                        number += 6;
                    }

                    true
                }
            }
        )*
    }
}

impl_is_prime_trait!(i8 u8 i16 u16 i32 u32 i64 u64);

pub struct PrimeSieve(Vec<bool>);

impl PrimeSieve {
    #[inline]
    pub fn new(limit: usize) -> Self {
        let mut sieve = vec![true; limit + 1];
        sieve[0] = false;
        sieve[1] = false;

        for i in 2..=limit.sqrt() {
            for j in (i * i..=limit).step_by(i) {
                if sieve[j] {
                    sieve[j] = false;
                }
            }
        }

        Self(sieve)
    }

    #[inline]
    pub fn get_primes(self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|&(_, &is_prime)| is_prime)
            .map(|(p, _)| p)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_5_is_prime() {
        assert!(5.is_prime());
    }

    #[test]
    fn sieve_of_hundred() {
        let sieve = PrimeSieve::new(100);

        assert_eq!(
            sieve.get_primes(),
            &[
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        )
    }
}
