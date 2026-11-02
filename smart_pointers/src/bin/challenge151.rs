use crate::Room::{Nil, Recurse};

// WITHOUT 'use'
// let room = Room::Recurse(1, Box::new(Room::Recurse(2, Box::new(Room::Nil))));

fn main() {
    let room = Recurse(1, Box::new(Recurse(2, Box::new(Nil))));

    // You can read the nested data without taking ownership via references
    // Borrow 'next_box' as a reference (&Box<Room>)
    if let Recurse(id, ref next_box) = room {
        println!("Current room: {id}");

        // Dereference the reference to peek into the heap without moving
        if let Recurse(inner_id, _) = &**next_box {
            println!("Next room: {inner_id}");
        }
    }

    // Alternatively, move the original room (next_box)
    // Match outer enum on stack frame
    if let Recurse(id, next_box) = room {
        println!("Current room: {id}");

        // Dereference 'next_box' to move the inner 'Room' off the heap
        let inner_room: Room = *next_box;

        // Match on the inner room now sitting on the stack frame
        if let Recurse(inner_id, _) = inner_room {
            println!("Next room: {inner_id}");
        }
    }

    // // You can even do nested pattern matching (albeit a bit more difficult to think of)
    // if let Recurse(1, next_box) = room {
    //     if let Recurse(2, _) = *next_box {
    //         println!("Successfully peeked into room 1 and room 2!");
    //     }
    // }
}

enum Room {
    Recurse(i32, Box<Room>),
    Nil,
}

enum Term {
    True,
    False,
    Not(Box<Term>),
}

impl Term {
    pub fn simplify(self) -> Term {
        match self {
            // Case 1: Match `Not`, move `content` (Box<Term>) out
            Term::Not(content) => match *content {
                // Double Negation Elimination: Not(Not(x)) -> x
                Term::Not(inner_content) => inner_content.simplify(),

                // Standard Negations
                Term::True => Term::False,
                Term::False => Term::True,
            },
            // Base cases (True, False) remain unchanged
            other => other,
        }
    }
}
