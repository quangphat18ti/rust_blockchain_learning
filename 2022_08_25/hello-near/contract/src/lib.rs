// dev-1661425352219-77358215552497
#![allow(unused)]
use std::collections::HashMap;
use std::hash::Hash;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
 
 
 // Define the contract structure
 #[near_bindgen]
 #[derive(BorshDeserialize, BorshSerialize,  Debug)]
// #[derive(Debug, )]
pub struct Contract {
    todo_list : Vec<Todo>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,  Debug)]
// #[derive(Debug, )]
pub struct Todo{
    work: String,
    deadline: String,
}
impl Todo{
    fn new(w: String, dl: String) -> Self {
        Self{
            work: w,
            deadline: dl,
        }
    }
}
impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool{
        self.work == other.work && self.deadline == other.deadline
    }
}
impl Clone for Todo{
    fn clone(&self) -> Self {
        Self { work: self.work.clone(), deadline: self.deadline.clone()} 
    }
}
 
 // Define the default, which automatically initializes the contract
 impl Default for Contract{
    fn default() -> Self{
        Contract { todo_list: Vec::new(), }
    }
 }
 
 // Implement the contract structure
 #[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self{
        Self{
            todo_list: Vec::new(),
        }
    }
    pub fn get_list(&self) {
        let a = self.todo_list.clone();
        env::log(
            format!(
                "list of work\n{:#?}", a
            )
                .as_bytes(),
        );
    }

    pub fn add(&mut self, w: String, dl: String){
        let a = Todo::new(w, dl);
        self.todo_list.push(a);
    }

    pub fn delete(&mut self, w: String, dl: String) {
        let a = Todo::new(w, dl);
        let pos = self.todo_list.iter().position(|x| *x == a);
        match pos {
            Some(pos) => {
                self.todo_list.remove(pos);
                env::log(
                    format!(
                        "Remove is successfully!\n"
                    )
                        .as_bytes(),
                );
            }
            None => {
                env::log(
                    format!(
                        "Can't find this!!\nTry again."
                    )
                        .as_bytes(),
                );
            }
        }
    }
}
