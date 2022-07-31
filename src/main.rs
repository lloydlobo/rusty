use std::{
    collections::HashMap,
    convert::TryInto,
    fs::File,
    io::{self, prelude::*, BufRead, BufReader, Lines},
    path::Path,
};


fn main() {
    let path_fibo: &str = "compile_fib.txt";
    write_fs_compile_fibo(29, path_fibo.to_owned());
    let res_read: Result<(), io::Error> = read_bytes_write_buf(path_fibo.to_owned());
    let res_string_read: String = read_lines_ok(path_fibo);

    println!(" res_read: {:?}, res_string_read: {}", res_read,res_string_read);
}

/// Caches results in local storage txt
fn write_fs_compile_fibo(num: u128, path_file: String) {
    let compile_fib: String = compile_fibo_to_string(num);
    write_storage_local(&compile_fib, path_file);
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

/// Write to local storage using File
pub fn write_storage_local(string_to_write: &str, path_file: String) {
    let path: &Path = Path::new(&path_file);
    let display: std::path::Display = path.display();
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file: File = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    println!("file:create: {:?}", file);
    // Write the 'lorem_ipsum' strin to 'file', return `io::Result<()>`
    match file.write_all(string_to_write.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

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
