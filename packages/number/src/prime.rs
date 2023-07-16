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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_5_is_prime() {
        assert!(5.is_prime());
    }
}
