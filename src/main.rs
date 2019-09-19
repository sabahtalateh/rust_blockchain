#[macro_use]
extern crate serde;

use std::io;
use std::io::{Write};
use crate::blockchain::Chain;
use std::process::exit;

mod blockchain;

fn main() {
    let miner_addr = read_string("miner addr: ");
    let difficulty = read_u32("difficulty: ");

    println!("generating genesis");
    let mut blockchain = Chain::new(miner_addr, difficulty);
    blockchain.mine();
    loop {

        println!("menu:");
        println!("1) new transaction");
        println!("2) mine block");
        println!("3) print chain");
        println!("0) exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        match option.trim() {
            "0" => {
                println!("bue..");
                exit(0);
            },
            "1" => {
                let sender = read_string("sender: ");
                let receiver = read_string("receiver: ");
                let amount = read_f32("amount: ");

                blockchain.new_transaction(sender, receiver, amount);
            },
            "2" => {
                blockchain.mine();
                println!("block mined");
            },
            "3" => {
                println!("{:#?}", blockchain);
            },
            x => {
                println!("[{}] is unknown command. try again", x);
            }
        }
    }
}

fn read_f32(prompt: &str) -> f32 {
    let mut number_f32: f32 = 0.0;
    let mut parsed = false;
    while !parsed {
        let mut number_string = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut number_string).unwrap();
        if let Ok(d) = number_string.trim().parse::<f32>() {
            number_f32 = d;
            parsed = true;
        } else {
            println!("positive floating point number required");
        }
    }
    number_f32
}

fn read_u32(prompt: &str) -> u32 {
    let mut number_u32: u32 = 0;
    let mut parsed = false;
    while !parsed {
        let mut number_string = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut number_string).unwrap();
        if let Ok(d) = number_string.trim().parse::<u32>() {
            number_u32 = d;
            parsed = true;
        } else {
            println!("positive integer number required");
        }
    }
    number_u32
}

fn read_string(prompt: &str) -> String {
    let mut string = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut string).unwrap();
    string.trim().to_string()
}
