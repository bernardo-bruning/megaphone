use std::io::{self, Read};
use clap::{Arg, App};
fn main() {
    let matches = App::new("Megaphone")
        .version("0.1")
        .author("Bernardo de Oliveira Bruning <bernardo.bruning@gmail.com>")
        .about("This program allows you to send messages to communication system channels such as slack and discord, thus allowing you to notify data through an operating system data pipe.")
        .arg(Arg::with_name("token")
             .short("t")
             .long("token")
             .value_name("HASH")
             .required(true)
             .help("Access token for discord communications"))
        .arg(Arg::with_name("channel")
             .short("c")
             .long("channel")
             .required(true)
             .value_name("CHANNEL"))
        .get_matches();

    let mut message = String::new();
    io::stdin().read_to_string(&mut message).unwrap();
    let token = matches.value_of("token").unwrap();
    let channel = matches.value_of("channel").unwrap();
    println!("token: {}", token);
    println!("channel: {}", channel);
    println!("message: {}", message);
}
