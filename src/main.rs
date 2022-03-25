use std::{
    env::{self},
    error,
    fs::{self, File},
    io::{self, Read},
};

fn pwd() {
    let dir = env::current_dir().unwrap();
    println!("{}", dir.display())
}

fn echo(input: Vec<&str>) {
    let dim = input.len();
    if input[1] == "-n" {
        for i in 2..dim {
            print!("{} ", input[i]);
        }
    } else {
        for i in 1..dim {
            println!("{}", input[i]);
        }
    }
}

fn mkdir(input: Vec<&str>) {
    for i in 1..input.len() {
        fs::create_dir_all(input[i]);
    }
}

fn removedir(input: Vec<&str>) {
    for i in 1..input.len() {
        fs::remove_dir_all(input[i]);
    }
}

fn rm(input: Vec<&str>) {
    if input[1] == "-r" {
        for i in 2..input.len() {
            fs::remove_file(input[i]);
        }
    }
    if input[1] == "-R" {
        for i in 2..input.len() {
            fs::remove_file(input[i]);
        }
    }
    if input[1] == "--recursive" {
        for i in 2..input.len() {
            fs::remove_file(input[i]);
        }
    }
    // if input[1] == "-d"{

    // }
}

fn touch(input: Vec<&str>) {
    for i in 1..input.len() {
        let mut file = File::create(input[i]);
        // match file{
        //     Ok(file)=>file,
        //     Err(e)=>{
        //         println!("Error {}",e);
        //         std::process::exit(-100);
        // }
    }
}

fn cat(input: Vec<&str>) {
    for i in 1..input.len() {
        let mut file = File::open(input[i]).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s);
        println!("{} in {}", s, input[i]);
    }
}

//de rezolvat
fn ls(input: Vec<&str>) {
    fs::read_dir(input[1]);
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let input: Vec<&str> = input.split_whitespace().collect();
    match input[0] {
        "pwd" => pwd(),
        "echo" => echo(input),
        "mkdir" => mkdir(input),
        "rmdir" => removedir(input),
        "touch" => touch(input),
        "rm" => rm(input),
        "cat" => cat(input),
        _ => std::process::exit(-1),
    }
}
