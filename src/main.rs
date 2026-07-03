#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]



use std::io;
use std::env;
use std::process;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;



#[derive(Serialize, Deserialize, Debug)]
struct JsonMachineDescription {

    name: String,
    alphabet: Vec<String>,
    blank: String,
	states: Vec<String>,
	initial: String,
	finals: Vec<String>,
	transitions: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct Transition {

    pub read: char,
    pub to_state: String,
    pub write: char,
    pub action: bool,
}

#[derive(Deserialize, Debug)]
pub struct Machine {

    pub name: String,
    pub alphabet: Vec<String>,
    pub blank: char,
    pub states: Vec<String>,
    pub initial: String,
    pub finals: Vec<String>,
    pub transitions: HashMap<String, Vec<Transition>>,
}





fn help_msg(args: &[String]) {
	println!(
"usage: ft_turing [-h] jsonfile input

positional arguments:
    jsonfile            json description of the machine
    
    input               input of the machine

optional arguments:
    -h, --help	show this help message and exit");

}

fn err_msg(args: &[String]) {
    println!(
"error: no such command: `{}`

help: ./ft_turing --help", args[0]);
}


fn parse_json(args: &[String]) -> Machine {

    println!("load_machine");
    let content: String = std::fs::read_to_string(&args[0]).expect("could not read file");
    serde_json::from_str(&content).expect("invalid json")
}



fn main() {

    let args: Vec<String> = env::args().skip(1).collect();

    let args_len = args.len();

    let is_help  = args_len == 1 && args.iter().any(|arg| arg == "--help" || arg == "-h");

    let has_json = args_len == 2 && args[0].ends_with(".json");

    let malformed = args_len == 0 || !is_help || !has_json;


    let cmd_action: fn(&[String]) = if malformed {err_msg} else if is_help {help_msg} else {parse_json};
    

    cmd_action(&args);
}

