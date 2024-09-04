use rusqlite::{params, Connection, Result};
use std::error::Error;
include!("./task.rs");

const TABLE_NAME: &str = "tasks";

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn setup(conn: Connection) -> Self {
        Database { conn }
    }

    pub fn create_if_not_exist(&self) -> Result<bool, Box<dyn Error>> {
        let query = "
        SELECT name 
        FROM sqlite_master 
        WHERE type='table' AND name=?
        ";

        // Check if the table exists
        let exists = self
            .conn
            .prepare(query)?
            .query_row(params![TABLE_NAME], |row| {
                let name: String = row.get(0)?;
                Ok(name == TABLE_NAME)
            })
            .unwrap_or(false);

        // If the table does not exist, create it
        if !exists {
            self.conn.execute(
                "CREATE TABLE tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    body TEXT,
                    priority TEXT NOT NULL
                )",
                [],
            )?;
        }

        Ok(exists)
    }

    pub fn insert_task(&self, task: Task) -> Result<bool, Box<dyn Error>> {
        let query = "
        INSERT INTO tasks (name, body, priority) VALUES (?1, ?2, ?3);
        ";
        let priority = match task.priority {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        };

        let rows_affected = self
            .conn
            .execute(query, params![task.name, task.body, priority])?;

        Ok(rows_affected > 0)
    }

    pub fn select_all_tasks(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let query = "SELECT name, body, priority FROM tasks";
        let mut stmt = self.conn.prepare(query)?;

        // Use query_map to map rows to Task structs
        let rows = stmt.query_map([], |row| {
            Ok(Task {
                name: row.get(0)?,
                body: row.get(1)?,
                priority: match row.get::<_, String>(2)?.as_str() {
                    "High" => Priority::High,
                    "Medium" => Priority::Medium,
                    "Low" => Priority::Low,
                    _ => {
                        return Err(rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Text,
                            "Unknown priority".into(),
                        ));
                    }
                },
            })
        })?;

        // Collect the rows into a Vec
        let result: Result<Vec<Task>, rusqlite::Error> = rows.collect();
        result.map_err(From::from)
    }
}
