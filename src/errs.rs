use std::error::Error;

pub fn print_err<E: Error>(client: bool, err: E, msg: &str){
    let entity = if client {
        "client"
    } else {
        "server"
    };
    let error_msg = format!("{entity}: ERR: {err}\n{msg}");
    println!("{error_msg}");
}

pub fn panic_err<E: Error>(client: bool, err: E, msg: &str){
    let entity = if client {
        "client"
    } else {
        "server"
    };
    let error_msg = format!("{entity}: ERR: {err}\n{msg}");
    panic!("{error_msg}");
}