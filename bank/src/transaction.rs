use rusqlite::{Connection, Result};
use crate::client::Client;

fn deposit(value: f64, origin_client: i32, conn: &mut Connection) -> Result<()> {
    conn.execute(r#"
    UPDATE client
    SET balance = balance + ?
    WHERE id = ?
    "#, (value, origin_client))?;

    Ok(())
    }

fn withdraw(value: f64, origin_client: i32, conn: &mut Connection) -> Result<()> {
    conn.execute(r#"
    UPDATE client
    SET balance = balance - ?
    WHERE id = ?
    "#, (value, origin_client))?;

    Ok(())
    }

fn transfer(value: f64, origin_client: i32, destination_client: i32, conn: &mut Connection) -> Result<()> {
    conn.execute(r#"
    UPDATE client
    SET balance = balance - ?
    WHERE id = ?;

    UPDATE client
    SET balance = balance + ?
    WHERE id = ?;
    "#,
    (value, origin_client, value, destination_client))?;

    Ok(())
}

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
    destination_client: Option<&'a mut Client<'a>>,
    value: f64, 
    operation: Operation,
    conn: &'a Connection,
}

impl<'a> Transaction<'a> {
    pub fn new(origin: &'a mut Client<'a>,
                destination: Option<&'a mut Client<'a>>,
                value: f64,
                operation: Operation,
                conn: &'a Connection) -> Result<Transaction<'a>> {  

                    
        conn.execute(
    r#"INSERT INTO "transaction"(origin_id, destination_id, value, operation)
            VALUES (?, ?, ?, ?);
        "#,

        (origin.id, destination.as_ref().map_or(None, |d| Some(d.id)), value, operation.to_str())
    )?;


    Ok(Transaction{origin_client: origin,
                        destination_client: destination,
                        value: value,
                        operation: operation,
                        conn: conn})
    }

}
