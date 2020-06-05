use std::io::{self, BufRead, BufReader};
use serenity::client::{EventHandler, Client};
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serde_json::json;
use clap::{Arg, App};

struct Handler {
    channel: u64
}

impl EventHandler for Handler {
    fn ready(&self, context:Context, ready:Ready) {
        println!("{} is connected!", ready.user.name);
        println!("{:?}", ready);

        let mut bufferio = BufReader::new(io::stdin());
        let mut message = String::new();
        while bufferio.read_line(&mut message).unwrap() != 0 {
            println!("channel: {}", self.channel);
            println!("message: {}", message);
            let send_message = json!({
                "content": message,
                "tts": false
            });
            context.http.send_message(self.channel, &send_message).expect("Error to send message for channel");
            message = String::new();
        }
    }
}

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

    let token = matches.value_of("token").unwrap();
    let channel = matches.value_of("channel").unwrap().parse::<u64>().unwrap();
    let mut client = Client::new(&token, Handler{channel}).unwrap();
    
    client.start().expect("Error to connect client");
}
