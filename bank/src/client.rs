use std::time::SystemTime;
use rusqlite::{Connection, Result};

fn query_client(name: &str, password: &str, conn: &Connection) -> Result<(i32, f64)> {
    let id: i32 = conn.query_row(r#"
    SELECT * FROM client
    WHERE name = ?
    AND password = ?
    "#,
    [name.trim(), password.trim()],
    |row| row.get(0)
    )?;

    let balance: f64 = conn.query_row(r#"
    SELECT * FROM client
    WHERE name = ?
    AND password = ?
        "#,
    [name.trim(), password.trim()],
    |row| row.get(3)
    )?;

        

    Ok((id, balance))
}

pub struct Client<'a> {
    pub id: i32,
    name: &'a str,
    password: &'a str,
    pub balance: f64,
    creation_date: SystemTime,
    conn: &'a Connection,
}

impl<'a> Client<'a> {
    pub fn new(name: &'a str, password: &'a str, conn: &'a Connection, save: bool) -> Result<Client<'a>> {
        if save {
            conn.execute(r#"
            INSERT INTO client(name, password)
            VALUES (?, ?);
            "#, [name, password])?;
        }

        let id: i32 = conn.query_row(r#"
        SELECT MAX(id) FROM client;
        "#,
        [],
        |row| row.get(0)
        )?;

        Ok(Client {id, name, password, balance: 0.0, creation_date: SystemTime::now(), conn: conn })
    }


    pub fn auth(name: &'a str, password: &'a str, conn: &'a Connection) -> Result<Client<'a>> {
        let data:(i32, f64) = query_client(name, password, conn)?;

        let id: i32 = data.0;
        let balance: f64 = data.1;

        Ok(Client {id: id, name, password, balance: balance, creation_date: SystemTime::now(), conn})
    }

    pub fn search_client(name: &str, conn: &Connection) -> Result<Client<'a>> {
        

        Ok(Client { id: (), name, password: (), balance: (), creation_date: (), conn })
    }
}