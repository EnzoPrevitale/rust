use std::{io::{self, Write}, time::SystemTime};
use rusqlite::{Connection, Result};

mod client;
use client::Client;
mod transaction;
use transaction::Transaction;

fn create_tables(conn: &Connection) -> Result<()>{
    conn.execute(r#"CREATE TABLE IF NOT EXISTS client (
        id INTEGER PRIMARY KEY,
        name VARCHAR(50) UNIQUE NOT NULL,
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

pub fn input(content: &str) -> &str {
    print!(content);
    
    io::stdout().flush().unwrap();

    let mut text = String::new();

    io::stdin().read_line(&mut text).expect("Input error.");

    &text
}

fn main() -> Result<()> {
    let conn = Connection::open("database.db")?;

    create_tables(&conn)?;

    loop {
        println!("-=-=-=- BANCO -=-=-=-");
        println!("[1] Fazer login");
        println!("[2] Fazer cadastro");
        println!("[3] Sair");

        let option: &str = input("Escolha: ");

        if option == "1" {
            loop {
                println!("[1] Deposit");
                println!("[2] Withdraw");
                println!("[3] Transfer");
                println!("[4] Exit");

                let option: &str = input("Escolha: ");

                if option == "1" {
                    let name: &str = input("Digite o nome: ");
                }
            }
        } else if option == "2" {
            let name: &str = input("Nome: ");
            let password: &str = input("Senha");

            let client: Client = Client::new(name, password, &mut conn);
        }
    }

    Ok(())
}
