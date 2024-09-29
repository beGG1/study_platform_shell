use std::io::{stdin, stdout, Write};

use crate::api;

pub async fn platform_login() -> bool {
    print!("> Login: ");
    stdout().flush().unwrap();

    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();

    print!("> Password: ");
    stdout().flush().unwrap();

    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();

    match api::auth(username, password).await {
        Ok(res)=> {
            let res = res.status().as_u16();
            if res != 200 {
                return false;
            }
            return true
        },
        Err(err)=> {
            println!("{}", err);
            return false;
        },
    }
}