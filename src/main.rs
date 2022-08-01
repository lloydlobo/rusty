use std::{
    collections::HashMap,
    convert::TryInto,
    fs::File,
    io::{self, prelude::*, BufRead, BufReader, Lines},
    path::Path,
};

pub(crate) mod fibo;
pub(crate) mod memoize;
pub(crate) mod std_file;

pub(crate) use crate::{
    fibo::memoize_fibo, memoize::other_memoize as other_memoize,
    std_file::write_storage_local::write_storage_local,
};

fn main() {
    let path_fibo: &str = "compile_fib.txt";
    let path_cache: &str = "cache.txt";
    let path_memo: &str = "memoize.txt";
    let num: u128 = 20;

    let fibo_num_u128: u128 = fibo::fibo_memoize::memoize_fibo(num);
    write_fs_compile_fibo(path_cache.to_owned(), fibo_num_u128.to_string());

    let function: String = compile_fibo_to_string(num);
    write_fs_compile_fibo(path_fibo.to_owned(), function);
    let res_read: Result<(), io::Error> = read_bytes_write_buf(path_fibo.to_owned());
    let res_string_read: String = read_lines_ok(path_fibo);

    println!(
        " res_read: {:?}, res_string_read: {}",
        res_read, res_string_read
    );
    let memoize_part_2: u128 = other_memoize(fibo_num_u128);
    write_fs_compile_fibo(path_memo.to_owned(), memoize_part_2.to_string());
}

/// Caches results in local storage txt
fn write_fs_compile_fibo(path_file: String, func: String) {
    // let compile_fib: String = compile_fibo_to_string(num);
    write_storage_local(&func, path_file);
}

/// Pushes fibonacci numbers into a string separated by a comma `,`
fn compile_fibo_to_string(num: u128) -> String {
    let mut res_string = String::new();
    for i in 1..num {
        let curr_fibo_str: String = memoize_fibo(i).to_string();
        res_string.push_str(&curr_fibo_str);
        if i < num - 1 && !curr_fibo_str.is_empty() {
            res_string.push(',');
        }
    }

    res_string
}

// The method lines() returns an iterator over the lines of a file.
// File::open expects a generic, AsRef<Path>. That's what read_lines() expects as input.
// |line: Result<String, io::Error>|
fn read_lines_ok(filename: &str) -> std::string::String {
    let mut res_string = String::new();
    for line in match read_lines::<&str>(filename) {
        Ok(it) => it,
        _ => panic!(), // return (Originally)
    } {
        match line {
            Ok(result_read) => {
                // println!("result_read: {}", result_read);
                res_string.push_str(&result_read);
            }
            Err(_) => panic!(),
        }
    }

    res_string
}

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file_open: File = File::open(filename)?;
    Ok(BufReader::new(file_open).lines())
}

/// Read the local storage using File
pub fn read_bytes_write_buf(path_file: String) -> io::Result<()> {
    let mut buf = String::new();
    let mut file: File = File::open(path_file)?;
    let _: usize = file.read_to_string(&mut buf)?;

    Ok(())
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
            let prep_fibo_next: u128 = memoize_fibo(num); // so this is the fibo of prior num
                                                          // we want to store the sequences of fibo
                                                          // so write it to local storage
            if !self.memo.contains_key(&num) {
                self.memo.entry(num).or_insert(prep_fibo_next);
            }
            let memo_res: u128 = *self.memo.get(&num).unwrap();

            memo_res
        }
    }
    let res: u128 = MemoFibo::new(num).memo_fibo(num);

    res
}
