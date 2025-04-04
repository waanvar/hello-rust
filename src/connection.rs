use rusqlite::{params, Connection};

fn main() -> Result<(), rusqlite::Error> {
    // Replace these values with your Informix database connection details
    let conn: Connection = Connection::open_with_flags(
        "Driver={Informix};Host=132.146.222.2;Server=onlineimp;Protocol=onsoctcp;Database=lan;Uid=bck;Pwd=bck;",
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE,
    )?;

    // Example query
    let mut stmt = conn.prepare("SELECT * FROM useracl where user_id = 'wannavar'")?;
    let rows = stmt.query_map(params![], |row| {
        // You can access columns by name or index
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    // Iterate over the rows and print them
    for row in rows {
        let (id, name) = row?;
        println!("ID: {}, Name: {}", id, name);
    }

    Ok(())
}
