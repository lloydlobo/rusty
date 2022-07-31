use std::collections::HashMap;
use std::convert::TryInto;
// use crate::fibo::fibo_memoize::memoize_fibo;

/// .
///
/// # Panics
///
/// Panics if .
pub(crate) fn memoize(res: u128) -> u128 {

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
                let next = crate::fibo::fibo_memoize::memoize_fibo(res);
                self.memoize.entry(res).or_insert(next);
            };

            *self.memoize.get(&res).unwrap()
        }
    }

    Memo::new(res).memoize_this(res)
}
