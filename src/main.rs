use clap::{Parser, Subcommand};
use rusqlite::{Connection, OpenFlags};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    // #[error("unknown error")]
    // Unknown,
}

type Result<T, E = MyError> = std::result::Result<T, E>;

#[derive(Debug, Parser)]
#[clap(about = "puttanesca service", long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Test {
        #[clap(long, help="Database path")]
        path: String,
        #[clap(long, action, help="Create the database if not exists")]
        create: bool,
    },
}

/// sqlite opens databases lazily, so [`Connection::open_with_flags`] does not produce any error with invalid database file.
/// Executing a pragma function forces the database to open and check if it is valid.
fn check_valid_connection(conn: Connection) -> Result<Connection, rusqlite::Error> {
    conn.pragma_query(None,"schema_version", |_| Ok(()))?;
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

    let conn = Connection::open_with_flags(path, flags)
        .and_then(check_valid_connection);

    match conn {
        Ok(conn) => println!("DB path: {} (busy: {})", conn.path().unwrap_or("unknown"), conn.is_busy()),
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

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Test { path, create } => test_conn(&path, create),
    }
}
