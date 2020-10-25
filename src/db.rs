use lazy_static::lazy_static;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref DB: Mutex<Connection> = {
        let conn = Connection::open_in_memory().expect("Fail to open Sqlite connection!");

        conn.execute_batch(
            "CREATE TABLE songs (
                      id              INTEGER PRIMARY KEY,
                      path            TEXT NOT NULL,
                      title           TEXT NOT NULL,
                      artist          TEXT,
                      album_id        INTEGER
                      );
             CREATE TABLE albums (
                      id              INTEGER PRIMARY KEY,
                      title           TEXT NOT NULL,
                      artist          TEXT,
                      cover           BLOB
                      );",
        )
        .expect("fail to create table!");
        Mutex::new(conn)
    };
}
