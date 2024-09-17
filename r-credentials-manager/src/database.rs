use rusqlite::{params, Connection, Result};
use std::error::Error;

const TABLE_NAME: &str = "credentials";
include!("./utils.rs");

pub struct Credential {
    pub id: String,
    pub username: String,
    pub usage_desc: String,
    pub password: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn setup(conn: Connection) -> Self {
        Database { conn }
    }

    pub fn check_master(&self, plain_master: String) -> Result<bool, Box<dyn Error>> {
        let query = "SELECT encrypted_password FROM credentials WHERE id = 1";
        let hashed_master = self
            .conn
            .query_row(query, [], |row| row.get::<_, String>(0))?;

        if let hashed_master = hashed_master {
            let is_accessible = compare_hashes(hash_master(&plain_master), hashed_master);
            Ok(is_accessible)
        } else {
            Ok(false)
        }
    }

    pub fn store_master_hash(&self, plain_master: String) -> Result<bool, Box<dyn Error>> {
        let query = "
        INSERT INTO credentials (usage_desc, username, encrypted_password) VALUES (?1, ?2, ?3);
        ";

        let rows_affected = self.conn.execute(
            query,
            params![
                "the hashed master password",
                "master_password",
                hash_master(&plain_master)
            ],
        )?;

        Ok(rows_affected > 0)
    }

    pub fn create_if_not_exist(&self) -> Result<bool, Box<dyn Error>> {
        let query = "
        SELECT name 
        FROM sqlite_master 
        WHERE type = 'table' AND name = ?
        ";

        let exists = self
            .conn
            .prepare(query)?
            .query_row(params![TABLE_NAME], |row| {
                let name: String = row.get(0)?;
                Ok(name == TABLE_NAME)
            })
            .unwrap_or(false);

        if !exists {
            self.conn.execute(
                "CREATE TABLE credentials (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    usage_desc TEXT NOT NULL,
                    username TEXT,
                    encrypted_password TEXT NOT NULL
                )",
                [],
            )?;
        }

        Ok(exists)
    }

    pub fn insert_creds(
        &self,
        credential: Credential,
        keyword: String,
    ) -> Result<bool, Box<dyn Error>> {
        let query = "
        INSERT INTO credentials (usage_desc, username, encrypted_password) VALUES (?1, ?2, ?3);
        ";

        let rows_affected = self.conn.execute(
            query,
            params![
                credential.usage_desc,
                credential.username,
                vigenere_encrypt(&credential.password, &keyword)
            ],
        )?;

        Ok(rows_affected > 0)
    }

    pub fn select_all_creds_all_info(
        &self,
        keyword: String,
    ) -> Result<Vec<Credential>, Box<dyn Error>> {
        let query = "SELECT * FROM credentials";
        let mut stmt = self.conn.prepare(query)?;

        let rows = stmt.query_map([], |row| {
            Ok(Credential {
                id: String::from(row.get::<_, u16>(0)?.to_string()),
                usage_desc: row.get(1)?,
                username: row.get(2)?,
                password: vigenere_decrypt(&row.get::<_, String>(3)?, &keyword),
            })
        })?;

        let result: Result<Vec<Credential>, rusqlite::Error> = rows.collect();
        result.map_err(From::from)
    }

    pub fn select_all_creds_usage_desc(&self) -> Result<Vec<Credential>, Box<dyn Error>> {
        let query = "SELECT * FROM credentials";
        let mut stmt = self.conn.prepare(query)?;

        let rows = stmt.query_map([], |row| {
            Ok(Credential {
                id: row.get::<_, u16>(0)?.to_string(),
                usage_desc: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
            })
        })?;

        let result: Result<Vec<Credential>, rusqlite::Error> = rows.collect();
        result.map_err(From::from)
    }

    pub fn select_cred(&self, id: String, keyword: String) -> Result<Credential, Box<dyn Error>> {
        let query = "SELECT * FROM credentials WHERE id = ?";

        let cred = self.conn.query_row(query, params![id], |row| {
            Ok(Credential {
                id: row.get::<_, String>(0)?,
                usage_desc: row.get::<_, String>(1)?,
                username: row.get::<_, String>(2)?,
                password: vigenere_decrypt(&row.get::<_, String>(3)?, &keyword),
            })
        })?;

        Ok(cred)
    }
}
