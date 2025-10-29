use rusqlite::{Connection, Result};
use crate::client::Client;

pub enum Operation {
    Deposit,
    Withdraw,
    Transfer,       
}

impl Operation {
    fn to_str(&self) -> &str {
        match self {
            Operation::Deposit => "Deposit",
            Operation::Withdraw => "Withdraw",
            Operation::Transfer => "Transfer",
        }
    }
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
                conn: &'a mut Connection) -> Result<Transaction<'a>> {

        conn.execute(
    r#"INSERT INTO "transaction"(origin_id, destination_id, value, operation)
            VALUES (?, ?, ?, ?);
        "#,
        (origin.id, destination.id, value, operation.to_str())
    )?;

        Ok(Transaction{origin_client: origin,
                    destination_client: destination,
                    value: value,
                    operation: operation,
                    conn: conn})
    }

    fn deposit(&self) -> Result<()> {
        self.conn.execute(r#"
        UPDATE client
        SET balance = balance + ?
        WHERE id = ?
        "#, (self.value, self.origin_client.id))?;

        Ok(())
    }

    fn withdraw(&self) -> Result<()> {
        self.conn.execute(r#"
        UPDATE client
        SET balance = balance - ?
        WHERE id = ?
        "#, (self.value, self.origin_client.id))?;

        Ok(())
    }

    fn transfer(&self) -> Result<()> {
        self.conn.execute(r#"
        UPDATE client
        SET balance = balance - ?
        WHERE id = ?;

        UPDATE client
        SET balance = balance + ?
        WHERE id = ?;
        "#,
        (self.value, self.origin_client.id, self.value, self.destination_client.id))?;

        Ok(())
    }
}
