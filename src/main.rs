use colored::*;
use std::{
    collections::HashMap,
    convert::TryInto,
    fs::File,
    io::{self, prelude::*, BufRead, BufReader, Lines},
    ops::ControlFlow,
    path::Path,
};

pub(crate) mod fibo;
pub(crate) mod memoize;
pub(crate) mod std_file;

#[allow(unused_imports)]
pub(crate) use crate::{
    fibo::memoize_fibo, memoize::other_memoize, std_file::write_storage_local::write_storage_local,
};

pub fn welcome_user() {
    println!("{}", "Welcome to fibonacci generator!".blue());
    println!("{}", "You have to pick a index to fibonaize...\n".yellow());
}

pub fn loop_user_inputs() {
    loop {
        println!("{} ", "\nType a number between 0 and 42\n".cyan().bold());
        let mut input_new_string: String = String::new();
        // Give it text interactively by running the executable directly,
        // in which case it will wait for the Enter key to be pressed before continuing
        io::stdin()
            .read_line(&mut input_new_string)
            .expect("Failed to read your input number!");

        let input_trim: &str = input_new_string.trim();

        let _input_fibo_num: u128 = match Some(input_trim) {
            Some(can_break)
                if match_input_cli_user(can_break.to_owned()) == ControlFlow::Break(()) =>
            {
                break
            }

            Some(number_possible) if input_is_number(number_possible, input_trim) => {
                println!("You entered number: {}", number_possible);
                input_trim.parse().unwrap()
            }
            Some(_) => continue,
            None => panic!(),
        };

        if let ControlFlow::Break(_) = match_input_cli_user(input_trim.to_owned()) {
            break;
        }

        println!("You entered: {}", input_trim);
    }
}

pub fn input_is_number(number_possible: &str, input_trim: &str) -> bool {
    ({
        match number_possible.trim().parse::<u128>() {
            Ok(t) => t,
            Err(_) => {
                println!("Please enter a number");
                0
            }
        }
    }) == {
        match input_trim.parse() {
            Ok(t) => t,
            Err(_) => {
                println!("Please enter a number");
                1
            }
        }
    }
}
// let input_cli_user: u128 = match input_new_string.trim().parse() { Ok(number) => number, Err(other) => { println!("Not a number, {}", other); let input_num_to_str = other.to_string(); input_num_to_str } };

fn match_input_cli_user(input: String) -> ControlFlow<()> {
    const OPT_YES: &str = "y";
    const OPT_NO: &str = "n";
    const OPT_ALL: &str = "a";

    println!("input: {}", input);
    match Some(input) {
        Some(yes) if yes == OPT_YES => {
            println!("{}", yes);
            ControlFlow::Continue(())
        }
        Some(no) if no == OPT_NO => {
            println!("{}", no);
            ControlFlow::Break(())
        }
        Some(all) if all == OPT_ALL => {
            println!("{}", all);
            ControlFlow::Continue(())
        }
        None => panic!(),
        _ => ControlFlow::Continue(()),
    }
}

fn main() {
    welcome_user();
    loop_user_inputs();
    system_fibo();
}

/////////////////////////////////////////////////////////////
// start:           --- Helper Functions ---
/////////////////////////////////////////////////////////////

fn system_fibo() {
    let path_fibo: &str = "compile_fib.txt";
    let path_cache: &str = "cache.txt";
    #[allow(unused_variables)]
    let path_memo: &str = "memoize.txt";
    let num: u128 = 20;

    let fibo_num_u128: u128 = fibo::fibo_memoize::memoize_fibo(num);
    write_fs_compile_fibo(path_cache.to_owned(), fibo_num_u128.to_string());

    let function: String = compile_fibo_to_string(num);
    write_fs_compile_fibo(path_fibo.to_owned(), function);
    let res_read: Result<(), io::Error> = read_bytes_write_buf(path_fibo.to_owned());
    let res_string_read: String = read_lines_ok(path_fibo);

    // let memoize_part_2: u128 = other_memoize(fibo_num_u128);
    // write_fs_compile_fibo(path_memo.to_owned(), memoize_part_2.to_string());
    println!(
        " res_read: {:?}, res_string_read: {}",
        res_read, res_string_read
    );
}
/////////////////////////////////////////////////////////////
// end:             --- Helper Functions ---
/////////////////////////////////////////////////////////////

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
