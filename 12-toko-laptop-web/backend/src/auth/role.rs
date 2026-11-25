use std::{fmt, str::FromStr};

use axum::http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Buyer,
    Restocker,
}

impl UserRole {
    pub fn sebagai_teks(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Buyer => "buyer",
            Self::Restocker => "restocker",
        }
    }

    pub fn boleh_mengakses(self, method: &Method, path: &str) -> bool {
        if matches!(path, "/api/auth/session" | "/api/auth/logout") {
            return true;
        }

        match self {
            Self::Admin => true,
            Self::Buyer => {
                (*method == Method::GET
                    && (path == "/api/laptops" || path.starts_with("/api/laptops/")))
                    || (*method == Method::POST
                        && path.starts_with("/api/laptops/")
                        && path.ends_with("/buy"))
            }
            Self::Restocker => {
                (*method == Method::GET
                    && (path == "/api/laptops" || path.starts_with("/api/laptops/")))
                    || (*method == Method::PATCH && path.starts_with("/api/laptops/"))
            }
        }
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.sebagai_teks())
    }
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role {
            "admin" => Ok(Self::Admin),
            "buyer" => Ok(Self::Buyer),
            "restocker" => Ok(Self::Restocker),
            _ => Err(format!("Role pengguna tidak dikenal: {role}")),
        }
    }
}
