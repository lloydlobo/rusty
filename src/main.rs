use colored::*;
use std::{
    collections::HashMap,
    convert::TryInto,
    fs::{read_to_string, File},
    io::{self, prelude::*, BufRead, BufReader, Lines},
    num::ParseIntError,
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

/////////////////////////////////////////////////////////////
// start:           --- Main Function ---
/////////////////////////////////////////////////////////////
const PATH: &str = "loop_user.txt";

fn main() {
    welcome_user();
    let res_loop_input: u128 = loop_user_inputs();

    if File::open(PATH).is_err() {
        let first_write = write_to_file(res_loop_input, PATH);
        println!("first_write_ok: {:?}", first_write);
    } else {
        let write_result: Result<(), io::Error> = write_to_file(res_loop_input, PATH);
        println!("write_ok: {:?}", write_result);

        let read_result = read_file_buf(PATH);
        println!("read_result: {:?}", read_result);
    }

    // let read: i32 = cli_main(PATH).unwrap(); println!("read: {:?}", read);
    // system_fibo();
}

fn read_file_buf(file_name: &str) -> std::io::Result<u128> {
    let bytes_usize;
    let file = File::open(file_name)?;

    let mut buf_reader: BufReader<File> = BufReader::new(file);
    let mut contents: String = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(it) => {
            bytes_usize = it;
            bytes_usize
        }
        Err(err) => return Err(err),
    };

    let get_content: String = match read_to_string(&file_name) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    let parse_content: u128 = {
        match get_content.trim().parse::<u128>() {
            Ok(t) => t,
            Err(e) => panic!("called `Result::unwrap()` on an `Err` value {}", e),
        }
    };

    Ok(parse_content)
}

fn write_to_file(input: u128, file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    match file.write_all(input.to_string().as_bytes()) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    Ok(())
}
/////////////////////////////////////////////////////////////
// end:           --- Main Function ---
/////////////////////////////////////////////////////////////

pub fn welcome_user() {
    println!("{}", "Welcome to fibonacci generator!".blue());
    println!("{}", "You have to pick a index to fibonaize...\n".yellow());
}

pub fn get_num_from_loop(input_str: &str) -> u128 {
    let num: u128 = input_str.parse().unwrap();
    num
}

pub fn input_exit(can_break: &str) -> ControlFlow<()> {
    match_input_cli_user(can_break.to_owned())
}

pub fn parse_num(num_input: String) -> u128 {
    num_input.trim().parse::<u128>().unwrap()
}

fn print_fibo_from_input(input_str: &str, get_num: u128) -> u128 {
    let fibo_num_u128: u128 = fibo::fibo_memoize::memoize_fibo(get_num);
    println!("The fibonacci for: {} is: {}", input_str, fibo_num_u128);
    fibo_num_u128
}

pub fn input_is_integer(num: &str) -> bool {
    let integer = { num.to_owned().parse::<i128>().unwrap_or(0) };
    integer.is_positive() || integer.is_negative() || integer == 0
}

pub fn loop_user_inputs() -> u128 {
    let mut num_u: u128 = 0;
    let mut num_res: u128 = 1;

    loop {
        println!("{} ", "\nType a number between 0 and 42\n".cyan().bold());
        let mut input_string: String = String::new();

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read your input number!");

        let input: &str = input_string.trim();

        match Some(input) {
            Some(exit) if input_exit(exit) == ControlFlow::Break(()) => {
                println!("{} ", "\nExiting. Thank you!\n".bright_green().bold());
                break;
            }
            Some(num) if input_is_integer(num) => {
                let num_i: i128 = num.parse::<i128>().unwrap();
                match Some(num_i) {
                    Some(num_i) if num_i.is_negative() && num_i != 0 => {
                        println!("{}", "Enter a positive number between 0 & 42.".yellow());
                        continue;
                    }
                    Some(num_i) if num_i > 42 => {
                        println!("{}", "Enter a number between 0 & 42.".yellow());
                        continue;
                    }
                    Some(_) => (),
                    None => (),
                };

                match Some(num) {
                    Some(num) if input_is_number(num, input) => {
                        num_u = get_num_from_loop(num);
                        num_res = print_fibo_from_input(num, num_u);

                        num_u
                    }
                    Some(_) => continue,
                    None => continue,
                };
            }
            Some(_) => {
                println!("{}", "Please enter a number".red().bold());
                continue;
            }
            None => panic!(),
        };

        if let ControlFlow::Break(_) = input_exit(input) {
            println!("{} ", "\nExiting. Thank you!\n".bright_green().bold());
            break;
        }
        // println!("You entered: {}", input);
    }

    println!("Fibo for {} is {}", num_u, num_res);
    num_res
}

pub fn input_is_number(num: &str, input_trim: &str) -> bool {
    (match num.trim().parse::<u128>() {
        Ok(t) => t,
        Err(_) => {
            println!("Please enter a number");
            0
        }
    }) == { input_trim.parse().unwrap_or(1) }
}

fn match_input_cli_user(input: String) -> ControlFlow<()> {
    const OPT_YES: &str = "y";
    const OPT_NO: &str = "n";
    const OPT_ALL: &str = "a";

    // println!("input: {}", input);
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

/*
 *
use std::fs;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let mut contents = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let mut contents: String = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}
 *
 */

pub struct Cli;

#[derive(Debug)]
enum CliError {
    IoError(io::Error),
    ParseError(ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<ParseIntError> for CliError {
    fn from(error: ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn cli_main(file_name: &str) -> std::result::Result<i32, CliError> {
    impl Cli {
        pub fn new() -> Self {
            Self
        }

        fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
            let contents = read_to_string(&file_name)?;
            let num: i32 = contents.trim().parse()?;
            Ok(num)
        }
    }

    Cli::open_and_parse_file(file_name)
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
