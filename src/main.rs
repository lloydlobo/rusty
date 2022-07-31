use std::{
    collections::HashMap,
    convert::TryInto,
    fs::File,
    io::{self, prelude::*, BufRead},
    path::Path,
};

static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";

fn main() {
    let path_fibo: &str = "compile_fib.txt";
    write_storage_local(LOREM_IPSUM, "lorem_ipsum.txt".to_string());
    process_memo_memoize_fibo(4);
    write_fs_compile_fibo(39, path_fibo.to_owned());

    let res_read: Result<(), io::Error> = read_storage_local(path_fibo.to_owned());
    println!("res_read: {:?}", res_read);
    // let path_hosts: &str = "./hosts";
    read_lines_ok(path_fibo);
}

/// Caches results in local storage txt
fn write_fs_compile_fibo(num: u128, path_file: String) {
    let compile_fib: String = compile_fibo_to_string(num);
    let compile_fib_len: usize = compile_fib.len();
    println!("{} {}", path_file, compile_fib_len);

    write_storage_local(&compile_fib, path_file);
}

/// Pushes fibonacci numbers into a string separated by a comma `,`
fn compile_fibo_to_string(num: u128) -> String {
    let mut res_string: String = String::new();
    for i in 1..num {
        let curr_fibo_str: String = memoize_fibo(i).to_string();
        res_string.push_str(&curr_fibo_str);
        if i < num - 1 && !curr_fibo_str.is_empty() {
            res_string.push(',');
        }
    }
    println!("result: {}", res_string);

    res_string
}

// The method lines() returns an iterator over the lines of a file.
// File::open expects a generic, AsRef<Path>. That's what read_lines() expects as input.
fn read_lines_ok(filename: &str) {
    let lines = match read_lines(filename) {
        Ok(it) => it,
        _ => return,
    };
    for line in lines {
        match line {
            Ok(result_read) => {
                println!("result_read: {}", result_read);
            }
            Err(_) => todo!(),
        }
    }
}
/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Read the local storage using File
pub fn read_storage_local(path_file: String) -> io::Result<()> {
    // let path = Path::new(&path_file);
    // let display = path.display();
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
