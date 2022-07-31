use crate::{fibo::fibo_memoize::memoize_fibo, memo_memo_fibo};

#[allow(dead_code)]
pub fn process_memo_memoize_fibo(num: u128) {
    let mut arr_res_memo: Vec<u128> = Vec::new();
    let mut res_memo: u128 = 0;
    for i in 1..num {
        res_memo = memoize_fibo(i);
        println!("res_memo: {}", res_memo);
        arr_res_memo.push(res_memo);
    }
    let mut arr_memo_res_memo: Vec<u128> = Vec::new();
    let memo_res_memo: u128 = memo_memo_fibo(res_memo);
    for _ in arr_res_memo.iter().clone() {
        arr_memo_res_memo.push(memo_res_memo);
    }
    // println!("arr_res_memo: {:?}", arr_memo_res_memo.iter()); println!("memo_res_memo: {}", memo_res_memo);
    println!("arr_res: {:?}, {:?}", arr_res_memo, arr_memo_res_memo);
}
