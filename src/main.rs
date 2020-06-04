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
             .help("Access token for discord communications"))
        .get_matches();

    if let Some(token) = matches.value_of("token") {
        println!("token: {}", token);
    }
}
