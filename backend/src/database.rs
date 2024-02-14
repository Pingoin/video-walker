use std::{cell::RefCell, sync::Mutex};

use actix_web::{error, web, Error};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Statement;
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Debug)]
pub struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}
#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new() -> Self {
        // connect to SQLite DB
        let manager = SqliteConnectionManager::file("weather.db");
        let pool = Pool::new(manager).unwrap();
        Database { pool: pool }
    }

    pub async fn init(&self) {
        let conn = self.pool.get().unwrap();

        web::block(move || {
            // simulate an expensive query, see comments at top of main.rs

            conn.execute(
                "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
                (), // empty list of parameters.
            )
            .unwrap();
            let me = Person {
                id: 0,
                name: "Gaylord Nelson".to_string(),
                data: None,
            };
            conn.execute(
                "INSERT INTO person (name, data) VALUES (?1, ?2)",
                (&me.name, &me.data),
            )
            .unwrap();
        })
        .await
        .unwrap();
    }

    pub async fn get_persons(&self) -> Vec<Person> {
        let mut result = Vec::new();
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();

        let person_iter = stmt
            .query_map([], |row| {
                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    data: row.get(2)?,
                })
            })
            .unwrap();

        for person in person_iter {
            if let Ok(pers) = person {
                result.push(pers)
            }
        }

        result
    }
}
