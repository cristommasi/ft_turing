#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


mod types;
use types::Machine;
use std::io;

fn load_machine(path: &str) -> Machine {

    let content: String = std::fs::read_to_string(path).expect("could not read file");
    serde_json::from_str(&content).expect("invalid json")
}

fn main() {

    let machine: Machine = load_machine("unary_sub.json");
    
    println!("name:     {}", machine.name);
    println!("blank:    {}", machine.blank);
    println!("initial:  {}", machine.initial);
    println!("alphabet: {:?}", machine.alphabet);
    println!("states:   {:?}", machine.states);
    println!("finals:   {:?}", machine.finals);
    println!("transitions:");
    for (state, transitions) in &machine.transitions {
        println!("  {}:", state);
        for t in transitions {
            println!("    read: {} -> write: {}, to_state: {}, action: {}", 
                t.read, t.write, t.to_state, t.action);
        }
    }
}

