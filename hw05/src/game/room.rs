use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use super::curio::Curio;
use super::hall::Hall;

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {
    pub fn new(name: &str, contents: Vec<Curio>, wumpus: bool) -> Self {
	Room { name: name.to_string(), contents: contents, halls: Vec::new(), wumpus: wumpus }
    }

    pub fn add_hall(&mut self, hall: Rc<Hall>) {
	self.halls.push(hall);
    }

    pub fn neighbors_string(&self) -> String {
	let mut vec: Vec<String> = Vec::new();
	for h in &self.halls {
	    let o = h.other(&self);
	    let s: String = o.borrow().name.clone();
	    vec.push(s);
	}
	vec.join(" ")
    }
}
