use std::collections::HashMap;

fn main() {
    let res_memo = memoize_fibo(26);
    println!("res_memo: {}", res_memo);
}

pub fn memoize_fibo(num: u128) -> u128 {
    #[derive()]
    pub struct Fibo {
        memoize: HashMap<u128, u128>,
    }

    impl Fibo {
        pub fn new(num: u128) -> Fibo {
            let num_size: usize = num.try_into().unwrap();

            Fibo {
                memoize: HashMap::with_capacity(num_size),
            }
        }

        pub fn fibo(&mut self, num: u128) -> u128 {
            if num <= 2 {
                return 1;
            };
            let prev = self.fibo(num - 2);
            let curr = self.fibo(num - 1);
            let fibo_next = prev + curr;

            if !self.memoize.contains_key(&num) {
                self.memoize.entry(num).or_insert(fibo_next);
            }
            let res: u128 = *self.memoize.get(&num).unwrap();

            res
        }
    }

    Fibo::new(num).fibo(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memoize_fibo() {
        assert_eq!(memoize_fibo(2), 1u128);
        assert_eq!(memoize_fibo(10), 55);
    }

    #[test]
    fn test_new_impl() {
        assert_eq!(1, 1);
    }
}
