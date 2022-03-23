use std::{env::{args, self}, fs};

fn pwd(){
    let dir = env::current_dir().unwrap();
    println!("{}",dir.display());
}

// fn echo(){
//     let optiune:String = param[2];
//     let parametru:String = param[3];
        

// }

fn mkdir(nume:&String){
    let r = fs::create_dir_all(nume);
    match r {
        Err(e) => println!("error creating {}: {}", nume, e),
        Ok(_) => println!("created {}: OK", nume),
    }
}
fn main() {
    let param: Vec<String> = args().collect();
    
    let comanda = &param[1];
    let nume = &param[2];
  
    match comanda.as_str(){
        "pwd" => pwd(),
        //"echo" => echo(),
        "mkdir" => mkdir(nume),
        _=>std::process::exit(-1),
    }
}
