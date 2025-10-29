use rusqlite::{Connection, Result};
use crate::client::Client;

pub enum Operation {
    Deposit(f64),
    Withdraw(f64),
    Transfer(f64),       
}

pub struct Transaction<'a> {
    origin_client: &'a mut Client<'a>,
    destination_client: &'a mut Client<'a>,
    value: f64, 
    operation: Operation,
    conn: &'a mut Connection,
}

impl<'a> Transaction<'a> {
    pub fn new(origin: &'a mut Client<'a>,
                destination: &'a mut Client<'a>,
                value: f64,
                operation: Operation,
                conn: &'a Connection) -> Result<Transaction<'a>> {

        conn.execute(
    r#"INSERT INTO "transaction"(origin_id, destination_id, value, operation)
            VALUES (?, ?, ?, ?);
        "#,
        [origin.id, destination.id, value, operation]
    )?;

        Ok(Transaction{origin_client: origin,
                    destination_client: destination,
                    value: value,
                    operation: operation,
                    conn: conn})
    }

    pub fn deposit(&self) {
        self.conn.execute(r#"
        UPDATE client
        SET balance = balance + ?
        WHERE id = ?
        "#, [self.value, self.origin_client.id])?;
    }

    pub fn withdraw(&self) {
        self.conn.execute(r#"
        UPDATE client
        SET balance = balance - ?
        WHERE id = ?
        "#, [self.value, self.origin_client.id])?;
    }

    pub fn 
}