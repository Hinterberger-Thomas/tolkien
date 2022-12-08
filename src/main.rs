use core::panic;
use std::{env, fs};

fn main(){
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1){
        Some(path) => path,
        None => panic!("you need to provide a filepath")
    };

    let conents = fs::read_to_string(file_path)
    .unwrap();

    println!("{}",conents);
    
}