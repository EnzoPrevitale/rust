use std::{time::SystemTime, io};
use rusqlite::{Connection, Result};

mod client;
use client::Client;
mod transaction;
use transaction::Transaction;

fn create_tables(conn: &Connection) -> Result<()>{
    conn.execute(r#"CREATE TABLE IF NOT EXISTS client (
        id INTEGER PRIMARY KEY,
        name VARCHAR(50) NOT NULL,
        balance REAL NOT NULL DEFAULT 0,
        creation_date DATETIME DEFAULT NOW()
    );"#, [])?;

    conn.execute(r#"CREATE TABLE IF NOT EXISTS "transaction" (
        id INTEGER PRIMARY KEY,
        origin_id INTEGER NOT NULL,
        destination_id INTEGER NOT NULL,
        value REAL,
        operation VARCHAR(10),

        FOREIGN KEY (origin_id) REFERENCES client(id),
        FOREIGN KEY (destination_id) REFERENCES client(id)
    );"#, [])?;

    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("database.db")?;

    create_tables(&conn)?;

    Ok(())
}
