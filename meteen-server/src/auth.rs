use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use entity::prelude::*;
use sea_orm::prelude::*;
use sha2::Digest;

pub async fn is_valid_login(username: &str, password: &str, conn: DatabaseConnection) -> bool {
    let user = match User::find_by_id(username).one(&conn).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            eprintln!("Login failed for {}, no such user", username);
            return false;
        }
        Err(e) => {
            eprintln!(
                "Login failed for {}, error retrieving user: {}",
                username, e
            );
            return false;
        }
    };

    let hashed = hash_password(password, &user.password_salt);
    if hashed == user.password_hash {
        return true;
    }

    false
}

pub async fn check_auth_headers(
    conn: &DatabaseConnection,
    headers: &HeaderMap,
) -> Result<User, Response> {
    let login = match headers.get("Login") {
        Some(l) => l,
        None => {
            return Err((StatusCode::UNAUTHORIZED, "Please provide a login header").into_response())
        }
    };

    let login = match login.to_str() {
        Ok(l) => l,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Malformed login header").into_response()),
    };

    let mut split = login.split(":");
    let username = split.next();
    let password = split.next();

    let (username, password) = match (username, password) {
        (Some(username), Some(password)) => (username, password),
        _ => return Err((StatusCode::UNAUTHORIZED, "Malformed login header").into_response()),
    };

    let user = match User::find_by_id(username).one(conn).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            eprintln!("User \"{}\" not found", username);
            return Err(
                (StatusCode::UNAUTHORIZED, "Username or password incorrect").into_response()
            );
        }
        Err(e) => {
            eprintln!("Error while retrieving user: \"{}\": {}", username, e);
            return Err(
                (StatusCode::UNAUTHORIZED, "Username or password incorrect").into_response()
            );
        }
    };

    let hashed_input = hash_password(password, &user.password_salt);

    if hashed_input == user.password_hash {
        return Ok(User);
    }

    eprintln!("Wrong pasword for user: \"{}\"", username);
    return Err((StatusCode::UNAUTHORIZED, "Username or password incorrect").into_response());
}

pub fn hash_password(password: &str, salt: &str) -> Vec<u8> {
    let salted = format!("{}{}", password, salt);

    let mut hasher = sha2::Sha512::new();
    hasher.update(salted);
    let hashed = hasher.finalize().to_vec();

    return hashed;
}
