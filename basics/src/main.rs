use rusqlite::{Connection, Result};

fn main() {
    let conn = Connection::open("database.db")?;
}
