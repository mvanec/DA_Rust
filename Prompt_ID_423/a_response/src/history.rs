#![allow(unused)]

use std::marker;

struct History {
    seq: Vec<Part>,
    current_id: u32,
}

impl History {
    pub fn add(&mut self, tx: Option<Transaction>) {

        self.seq.push(
            self.create_new_part(tx)
        );
    }

    fn create_new_part(&self, tx: Option<Transaction>) -> Part {
        Part {
            id: self.current_id, // Immutable borrow of self.current_id
            transaction: tx,
        }
    }
}

#[derive(Debug)]
struct Part {
    id: u32,
    transaction: Option<Transaction>,
}

#[derive(Debug)]
pub struct Transaction;
