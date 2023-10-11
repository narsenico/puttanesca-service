use crate::Result;
use async_trait::async_trait;
use rusqlite::{Connection, OpenFlags};

use super::Processor;

pub struct SqliteProcessor {
    db_name: String,
}

impl SqliteProcessor {
    pub fn new(db_name: String) -> Result<Self> {
        todo!()
    }
}

#[async_trait]
impl Processor for SqliteProcessor {
    fn name(&self) -> String {
        String::from("Sqlite Processor")
    }

    async fn process(&self, hunter: std::sync::Arc<dyn crate::hunters::Hunter>) -> Result<()> {
        todo!()
    }
}

/// sqlite opens databases lazily, so [`Connection::open_with_flags`] does not produce any error with invalid database file.
/// Executing a pragma function forces the database to open and check if it is valid.
fn check_valid_connection(conn: Connection) -> Result<Connection, rusqlite::Error> {
    conn.pragma_query(None, "schema_version", |_| Ok(()))?;
    Ok(conn)
}

fn test_conn(path: &str, create: bool) -> Result<()> {
    let flags = if create {
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX
            | OpenFlags::SQLITE_OPEN_CREATE
    } else {
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX
    };

    let conn = Connection::open_with_flags(path, flags).and_then(check_valid_connection);

    match conn {
        Ok(conn) => println!(
            "DB path: {} (busy: {})",
            conn.path().unwrap_or("unknown"),
            conn.is_busy()
        ),
        Err(err) => match err.sqlite_error_code() {
            Some(rusqlite::ErrorCode::CannotOpen) => {
                println!("Cannot open {}, try use --create", path)
            }
            Some(err) => println!("Unexpected error: {:?}", err),
            None => println!("Unexpected error opening db"),
        },
    };

    Ok(())
}
