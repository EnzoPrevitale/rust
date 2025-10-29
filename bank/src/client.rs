use std::time::SystemTime;

pub struct Client<'a> {
    name: &'a str,
    balance: f64,
    creation_date: SystemTime,
}

impl<'a> Client<'a> {
    pub fn new(name: &str) -> Client {
        Client { name, balance: 0.0, creation_date: SystemTime::now() }
    }

    pub fn deposit(&mut self, value: f64) {
        self.balance += value;
        println!("R${} depositados na conta de {}. Saldo: {}.", value, self.name, self.balance);
    }
}