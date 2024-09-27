use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;

mod api;

#[tokio::main]
async fn main(){
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let _args = parts;

        match command {
            "pwd" => {
                match env::current_dir() {
                    Ok(path) => println!("Current dir: {}", path.display()),
                    Err(error) => println!("An error occured in pwd: {}", error),
                }                
            },
            "ping" => api::ping().await,
            "exit" => return,
            _ => continue,
            // command => {
            //     let child = Command::new(command)
            //         .args(args)
            //         .spawn();
                
            //     match child {
            //         Ok(mut child) => { child.wait(); },
            //         Err(e) => eprintln!("{}", e),
            //     }
            // }
        }
    }
}