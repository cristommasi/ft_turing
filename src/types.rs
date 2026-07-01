
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct Machine {

    pub name:        String,
    pub alphabet:    Vec<String>,
    pub blank:       String,
    pub states:      Vec<String>,
    pub initial:     String,
    pub finals:      Vec<String>,
    pub transitions: HashMap<String, Vec<Transition>>,
}

#[derive(Deserialize, Debug)]
pub struct Transition {

    pub read:     String,
    pub to_state: String,
    pub write:    String,
    pub action:   String,
}

