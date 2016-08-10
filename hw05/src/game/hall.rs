use std::cell::RefCell;
use std::rc::Rc;

use super::room::Room;

#[derive(Debug)]
pub struct Hall {
    pub left: Rc<RefCell<Room>>,
    pub right: Rc<RefCell<Room>>,
}

impl Hall {
    pub fn new(left: Rc<RefCell<Room>>, right: Rc<RefCell<Room>>) -> Hall {
	Hall { left: left.clone(), right: right.clone() }
    }

    /// Given a Room `room`, find the room at the other end of Hall `self`.
    pub fn other(&self, room: &Room) -> Rc<RefCell<Room>> {
	if room.name == self.left.borrow().name {
	    self.right.clone()
	} else if room.name == self.right.borrow().name {
	    self.left.clone()
	} else {
	    panic!("Invalid room in hall");
	}
    }

}
