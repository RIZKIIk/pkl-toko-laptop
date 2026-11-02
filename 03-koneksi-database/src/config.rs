use sqlx::postgres::PgConnectOptions;
use std::{env, error::Error};

pub struct AppConfig {
    db_host: String,
    db_port: u16,
    db_name: String,
    db_user: String,
    db_password: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            db_host: required_non_empty("DB_HOST")?,
            db_port: env::var("DB_PORT")?.parse()?,
            db_name: required_non_empty("DB_NAME")?,
            db_user: required_non_empty("DB_USER")?,
            db_password: env::var("DB_PASSWORD")?,
        })
    }

    pub fn database_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.db_host)
            .port(self.db_port)
            .database(&self.db_name)
            .username(&self.db_user)
            .password(&self.db_password)
    }

    pub fn database_name(&self) -> &str {
        &self.db_name
    }
}

fn required_non_empty(name: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let value = env::var(name)?;
    if value.trim().is_empty() {
        return Err(format!("{name} tidak boleh kosong").into());
    }
    Ok(value)
}
