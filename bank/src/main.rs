use std::{clone, io::{self, Write}};
use rusqlite::{Connection, Result};

mod client;
use client::Client;
mod transaction;
use transaction::Transaction;

fn create_tables(conn: &Connection) -> Result<()>{
    conn.execute(r#"CREATE TABLE IF NOT EXISTS client (
        id INTEGER PRIMARY KEY,
        name VARCHAR(50) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL,
        balance REAL NOT NULL DEFAULT 0,
        creation_date DATETIME DEFAULT CURRENT_TIMESTAMP
    );"#, [])?;

    conn.execute(r#"CREATE TABLE IF NOT EXISTS "transaction" (
        id INTEGER PRIMARY KEY,
        origin_id INTEGER NOT NULL,
        destination_id INTEGER,
        value REAL,
        operation VARCHAR(10),

        FOREIGN KEY (origin_id) REFERENCES client(id),
        FOREIGN KEY (destination_id) REFERENCES client(id)
    );"#, [])?;

    Ok(())
}

pub fn input(content: &str) -> String {
    print!("{}", content);
    
    io::stdout().flush().unwrap();

    let mut text = String::new();

    io::stdin().read_line(&mut text).expect("Input error.");

    text
}

pub fn float(value: String) -> f64 {
    let value: &str = value.trim();
    let value: f64 = value.parse().expect("Type error.");
    value
}

fn main() -> Result<()> {
    let mut conn = Connection::open("database.db")?;

    create_tables(&conn)?;

    loop {
        let mut client: Client;

        println!("-=-=-=- BANCO -=-=-=-");
        println!("[1] Fazer login");
        println!("[2] Fazer cadastro");
        println!("[3] Sair");

        
        let option: String = input("Escolha: ");
        let option: &str = option.trim();

        if option == "1" {
            loop {
                let name: String = input("Nome de usuário: ");
                let password: String = input("Senha: ");

                client = Client::auth(name.trim(), password.trim(), &conn)?;

                println!("[1] Deposit");
                println!("[2] Withdraw");
                println!("[3] Transfer");
                println!("[4] Exit");

                let option: String = input("Escolha: ");
                let option: &str = option.trim();

                if option == "1" {
                    let value = float(input("Digite o valor ser depositado: "));
                    let transaction: Transaction = Transaction::new(&mut client, None , value, transaction::Operation::Deposit, &conn)?;
                }
            }
        } else if option == "2" {
            let name: String = input("Nome: ");
            let password: String = input("Senha: ");

            client = Client::new(name.trim(), password.trim(), &mut conn, true)?;
        } else {
            print!("Break");
            break;
        }
    }

    Ok(())
}
