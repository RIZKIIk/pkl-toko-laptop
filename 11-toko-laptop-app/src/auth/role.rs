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
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Buyer => "buyer",
            Self::Restocker => "restocker",
        }
    }

    pub(crate) fn can_access(self, method: &Method, path: &str) -> bool {
        if matches!(path, "/api/auth/session" | "/api/auth/logout") {
            return true;
        }

        match self {
            Self::Admin => true,
            Self::Buyer => {
                *method == Method::GET
                    && (path == "/api/laptops" || path.starts_with("/api/laptops/"))
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
        formatter.write_str(self.as_str())
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

#[cfg(test)]
mod tests {
    use axum::http::Method;

    use super::UserRole;

    #[test]
    fn admin_can_access_all_protected_routes() {
        assert!(UserRole::Admin.can_access(&Method::DELETE, "/api/laptops/1"));
        assert!(UserRole::Admin.can_access(&Method::POST, "/api/laptops"));
        assert!(UserRole::Admin.can_access(&Method::PATCH, "/api/admin/users/andi/role"));
    }

    #[test]
    fn buyer_can_only_read_laptops_and_manage_own_session() {
        assert!(UserRole::Buyer.can_access(&Method::GET, "/api/laptops"));
        assert!(UserRole::Buyer.can_access(&Method::GET, "/api/laptops/1"));
        assert!(UserRole::Buyer.can_access(&Method::POST, "/api/auth/logout"));
        assert!(!UserRole::Buyer.can_access(&Method::POST, "/api/laptops"));
        assert!(!UserRole::Buyer.can_access(&Method::PATCH, "/api/laptops/1"));
        assert!(!UserRole::Buyer.can_access(&Method::DELETE, "/api/laptops/1"));
    }

    #[test]
    fn restocker_can_read_and_update_laptops_only() {
        assert!(UserRole::Restocker.can_access(&Method::GET, "/api/laptops"));
        assert!(UserRole::Restocker.can_access(&Method::GET, "/api/laptops/1"));
        assert!(UserRole::Restocker.can_access(&Method::PATCH, "/api/laptops/1"));
        assert!(UserRole::Restocker.can_access(&Method::POST, "/api/auth/logout"));
        assert!(!UserRole::Restocker.can_access(&Method::POST, "/api/laptops"));
        assert!(!UserRole::Restocker.can_access(&Method::DELETE, "/api/laptops/1"));
        assert!(!UserRole::Restocker.can_access(&Method::PATCH, "/api/admin/users/andi/role"));
    }
}
