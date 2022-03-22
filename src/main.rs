use std::env::{args, self};

fn pwd(){
    let dir = env::current_dir().unwrap();
    println!("{:?}",dir);
}

fn echo(){
    println!("{}", env::args().skip(1).collect()::Vec<_>>().join(" "));

}
fn main() {
    let param: Vec<String> = args().collect();
    
    let comanda = &param[1];
    // let nume = &param[2];

    match comanda.as_str(){
        "pwd" => pwd(),
        "echo" => echo(),
        _=>std::process::exit(-1),
    }
}
