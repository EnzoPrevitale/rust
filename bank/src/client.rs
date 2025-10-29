use std::time::SystemTime;
use rusqlite::{Connection, Result};

pub struct Client<'a> {
    name: &'a str,
    password: &'a str,
    balance: f64,
    creation_date: SystemTime,
    conn: &'a Connection,
}

impl<'a> Client<'a> {
    pub fn new(name: &str, password: &str, conn: &'a Connection) -> Client {
        Client { name, password, balance: 0.0, creation_date: SystemTime::now(), conn:conn }
    }

    pub fn encrypt(text: &str) {

    }

    pub fn deposit(&mut self, value: f64) {
        self.balance += value;
        println!("R${} depositados na conta de {}. Saldo: {}.", value, self.name, self.balance);
    }
}