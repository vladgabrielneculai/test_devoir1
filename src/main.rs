use std::{
    env::{self},
    fs::{self, File}, io,
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
    for i in 1..input.len(){
        fs::create_dir_all(input[i]);
    }
}

fn removedir(input: Vec<&str>){
    for i in 1..input.len(){
        fs::remove_dir_all(input[i]);
    }
}

fn rm(input: Vec<&str>){
    for i in 1..input.len(){
        fs::remove_file(input[i]);
    }
}

fn touch(input: Vec<&str>){
    for i in 1..input.len(){
        let mut file = File::create(input[i]).expect("Error creating file");
    }
}

//de rezolvat
fn ls(input:Vec<&str>){
    fs::read_dir(input[1]).unwrap();
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
            //"echo -n" => echo_n(nume),
            "mkdir" => mkdir(input),
            "rmdir" => removedir(input),
            _ => std::process::exit(-1),
        }
}
