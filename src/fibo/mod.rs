pub(crate) mod fibo_memoize;
pub(crate) mod process_memo_memoize_fibo;

pub(crate) use self::fibo_memoize::memoize_fibo;


pub(crate) use self::process_memo_memoize_fibo::process_memo_memoize_fibo;

// #[allow(dead_code)]
// pub(crate) fn main_fibo() {
//     println!("mod::fibo");
// }
// pub(crate) mod iter_fibonacci;
// pub(crate) mod memoized_fibonacci;
//
// pub(crate) use self::iter_fibonacci::iter_fibonacci;
// pub(crate) use self::iter_fibonacci::iter_fibonacci_return;
// pub(crate) use self::iter_fibonacci::main_return;
// pub(crate) use self::memoized_fibonacci::memoized_fibonacci;
