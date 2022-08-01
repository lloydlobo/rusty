use std::{collections::HashMap, convert::TryInto};
use crate::memoize_fibo;

pub(crate) fn other_memoize(res: u128) -> u128 {
    #[derive()]
    pub struct Memo {
        memoize: HashMap<u128, u128>,
    }

    impl Memo {
        pub fn new(res: u128) -> Memo {
            Memo {
                memoize: HashMap::with_capacity(TryInto::try_into(res).unwrap()),
            }
        }
        pub fn memoize_this(&mut self, res: u128) -> u128 {
            if !self.memoize.contains_key(&res) {
                let next = memoize_fibo(res);
                self.memoize.entry(res).or_insert(next);
            };

            *self.memoize.get(&res).unwrap()
        }
    }

    Memo::new(res).memoize_this(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memoize_fibo() {
        assert_eq!(other_memoize(2), 1u128);
        assert_eq!(other_memoize(10), 55);
    }

    #[test]
    fn test_new_impl() {
        assert_eq!(other_memoize(2), 1u128);
        assert_eq!(other_memoize(1), 1u128);
    }
}
