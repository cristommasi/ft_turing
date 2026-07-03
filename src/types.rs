
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;


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



