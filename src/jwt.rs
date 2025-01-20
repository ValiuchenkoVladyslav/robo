use crate::state::AppState;
use jsonwebtoken::{decode, encode, errors::Result as JwtResult, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  id: i32,
  exp: u64,
}

const JWT_ALGO: Algorithm = Algorithm::HS512;
const JWT_EXP: Duration = Duration::from_secs((24 * 60 * 60) * 32); // 32 days

pub fn create_jwt(id: i32) -> String {
  let time_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

  encode(
    &Header::new(JWT_ALGO),
    &Claims {
      id,
      exp: (time_now + JWT_EXP).as_secs(),
    },
    AppState::jwt_encode(),
  )
  .unwrap()
}

pub fn validate_jwt(token: &str) -> JwtResult<i32> {
  decode::<Claims>(token, AppState::jwt_decode(), &Validation::new(JWT_ALGO))
    .map(|data| data.claims.id)
}
