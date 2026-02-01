use rusqlite::{params, Connection, Result as SqliteResult};
use std::sync::Mutex;

use crate::config::Config;
use crate::errors::{AppError, AppResult};

/// Database connection pool wrapper
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Create new database instance
    pub fn new(config: &Config) -> AppResult<Self> {
        let conn = Connection::open(&config.database_url)
            .map_err(|e| AppError::Database(e))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Initialize database schema
    pub fn init_schema(&self) -> AppResult<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                public_key TEXT NOT NULL,
                encrypted_private_key TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .map_err(|e| AppError::Database(e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS master_wallet (
                id INTEGER PRIMARY KEY,
                public_key TEXT NOT NULL,
                encrypted_private_key TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .map_err(|e| AppError::Database(e))?;

        Ok(())
    }

    /// Save new user to database
    pub fn save_user(
        &self,
        email: &str,
        password_hash: &str,
        public_key: &str,
        encrypted_private_key: &str,
    ) -> AppResult<i64> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT INTO users (email, password_hash, public_key, encrypted_private_key) 
             VALUES (?1, ?2, ?3, ?4)",
            params![email, password_hash, public_key, encrypted_private_key],
        )
        .map_err(|e| AppError::Database(e))?;

        Ok(conn.last_insert_rowid())
    }

    /// Get user by email
    pub fn get_user_by_email(&self, email: &str) -> AppResult<UserRecord> {
        let conn = self.conn.lock().unwrap();

        conn.query_row(
            "SELECT id, email, password_hash, public_key, encrypted_private_key 
             FROM users WHERE email = ?1",
            params![email],
            |row| {
                Ok(UserRecord {
                    id: row.get(0)?,
                    email: row.get(1)?,
                    password_hash: row.get(2)?,
                    public_key: row.get(3)?,
                    encrypted_private_key: row.get(4)?,
                })
            },
        )
        .map_err(|e| AppError::Database(e))
    }

    /// Get user by ID
    pub fn get_user_by_id(&self, user_id: i64) -> AppResult<UserRecord> {
        let conn = self.conn.lock().unwrap();

        conn.query_row(
            "SELECT id, email, password_hash, public_key, encrypted_private_key 
             FROM users WHERE id = ?1",
            params![user_id],
            |row| {
                Ok(UserRecord {
                    id: row.get(0)?,
                    email: row.get(1)?,
                    password_hash: row.get(2)?,
                    public_key: row.get(3)?,
                    encrypted_private_key: row.get(4)?,
                })
            },
        )
        .map_err(|e| AppError::Database(e))
    }

    /// Check if email already exists
    pub fn email_exists(&self, email: &str) -> AppResult<bool> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM users WHERE email = ?1",
                params![email],
                |row| row.get(0),
            )
            .map_err(|e| AppError::Database(e))?;

        Ok(count > 0)
    }

    /// Get user by public key
    pub fn get_user_by_public_key(&self, public_key: &str) -> AppResult<UserRecord> {
        let conn = self.conn.lock().unwrap();

        conn.query_row(
            "SELECT id, email, password_hash, public_key, encrypted_private_key 
             FROM users WHERE public_key = ?1",
            params![public_key],
            |row| {
                Ok(UserRecord {
                    id: row.get(0)?,
                    email: row.get(1)?,
                    password_hash: row.get(2)?,
                    public_key: row.get(3)?,
                    encrypted_private_key: row.get(4)?,
                })
            },
        )
        .map_err(|e| AppError::Database(e))
    }
}

/// User record from database
#[derive(Debug, Clone)]
pub struct UserRecord {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub public_key: String,
    pub encrypted_private_key: String,
}

/// Legacy function for backward compatibility
pub fn init_db() {
    // This is kept for backward compatibility but should use Database::new() instead
}
