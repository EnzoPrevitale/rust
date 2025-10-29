use std::time::SystemTime;
use rusqlite::{Connection, Result};

pub struct Client<'a> {
    pub id: i32,
    name: &'a str,
    password: &'a str,
    pub balance: f64,
    creation_date: SystemTime,
    conn: &'a mut Connection,
}

impl<'a> Client<'a> {
    pub fn new(name: &'a str, password: &'a str, conn: &'a mut Connection) -> Result<Client<'a>> {
        conn.execute(r#"
        INSERT INTO client(name, password)
        VALUES (?, ?);
        "#, [name, password])?;

        let id: i32 = conn.query_row(r#"
        SELECT MAX(id) FROM client;
        "#,
        [],
        |row| row.get(0)
    )?;

        Ok(Client {id, name, password, balance: 0.0, creation_date: SystemTime::now(), conn: conn })
    }

    pub fn deposit(&mut self, value: f64) {
        self.balance += value;
        println!("R${} depositados na conta de {}. Saldo: {}.", value, self.name, self.balance);
    }
}