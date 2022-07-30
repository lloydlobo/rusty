use ::std::convert::TryInto;
use std::collections::HashMap;

fn main() {
    let mut arr_res_memo: Vec<u128> = Vec::new();
    let res_memo: u128 = memoize_fibo(6);
    println!("res_memo: {}", res_memo);
    arr_res_memo.push(res_memo);

    let mut arr_memo_res_memo: Vec<u128> = Vec::new();
    let memo_res_memo: u128 = memo_memo_fibo(res_memo);
    println!("memo_res_memo: {}", memo_res_memo);
    arr_memo_res_memo.push(memo_res_memo);

    println!("arr_res: {:?}, arr_memo_res_memo: {:?}", arr_res_memo, arr_memo_res_memo);
}

pub fn memo_memo_fibo(num: u128) -> u128 {
    pub struct MemoFibo {
        memo: HashMap<u128, u128>,
    }

    impl MemoFibo {
        pub fn new(num: u128) -> MemoFibo {
            MemoFibo {
                memo: HashMap::with_capacity(num.try_into().unwrap()),
            }
        }

        pub fn memo_fibo(&mut self, num: u128) -> u128 {
            let prep_fibo_next: u128 = memoize_fibo(num);
            if !self.memo.contains_key(&num) {
                self.memo.entry(num).or_insert(prep_fibo_next);
            } 
            // else {
            //     return prep_fibo_next;
            // }
            let memo_res: u128 = *self.memo.get(&num).unwrap();

            memo_res
        }
    }
    let res: u128 = MemoFibo::new(num).memo_fibo(num);

    res
}

pub fn memoize_fibo(num: u128) -> u128 {
    #[derive()]
    pub struct Fibo {
        memoize: HashMap<u128, u128>,
    }

    impl Fibo {
        pub fn new(num: u128) -> Fibo {
            let num_size: usize = TryInto::try_into(num).unwrap();

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
        assert_eq!(memoize_fibo(2), 1u128);
        assert_eq!(memoize_fibo(1), 1u128);
    }
}
