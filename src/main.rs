use std::env;
use std::io::{stdin, stdout, Write};
//use std::process::Command;

mod api;
mod auth;

fn pass() {
    // DO NOTHING
}

#[tokio::main]
async fn main(){
    while !auth::platform_login().await {
        println!("Auth failed. Please try again.")
    }
    println!("You successfully logged in");
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        match stdin().read_line(&mut input){
            Ok(_)=> pass(),
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }

        let mut parts = input.trim().split_whitespace();
        let command = parts.next();
        let command = match command {
            None => continue,
            Some(s) => s,
        };

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