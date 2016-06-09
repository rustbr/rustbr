extern crate crypto;
extern crate jwt;
extern crate uuid;
extern crate chrono;

static SECRET: &'static str = "some_secret_key";

use std::default::Default;
use self::uuid::Uuid;
use self::crypto::sha2::Sha256;
use self::jwt::{Header, Registered, Token};
use self::chrono::*;

#[derive(Debug)]
pub enum TokenError {
    InvalidToken,
    InvalidSignarue,
    Expired
}

#[derive(Debug)]
pub struct SessionToken {
    pub sub: Uuid,
    pub iat: DateTime<UTC>,
    pub exp: DateTime<UTC>
}

impl SessionToken {
    pub fn new(sub: Uuid) -> SessionToken {
        SessionToken {
            sub: sub,
            iat: UTC::now(),
            exp: UTC::now() + Duration::seconds(86_400)
        }
    }

    pub fn to_string(self) -> String {
        let header: Header = Default::default();
        let claims = Registered {
            sub: Some(self.sub.to_string()),
            iat: Some(self.iat.timestamp() as u64),
            exp: Some(self.exp.timestamp() as u64),
            ..Default::default()
        };
        let token = Token::new(header, claims);
        let jwt = token.signed(SECRET.as_bytes(), Sha256::new()).unwrap();

        jwt
    }

    pub fn from_string(token: String) -> Result<SessionToken, TokenError> {
        let session = match Token::<Header, Registered>::parse(&token) {
            Ok(v) => v,
            Err(_) => return Err(TokenError::InvalidToken)
        };

        if session.verify(SECRET.as_bytes(), Sha256::new()) {
            if session.claims.exp.unwrap() < UTC::now().timestamp() as u64 {
                Err(TokenError::Expired)
            } else {
                Ok(SessionToken {
                    sub: Uuid::parse_str(&session.claims.sub.unwrap()).unwrap(),
                    iat: UTC.timestamp(session.claims.iat.unwrap() as i64, 0),
                    exp: UTC.timestamp(session.claims.exp.unwrap() as i64, 0)
                })
            }
        } else {
            Err(TokenError::InvalidSignarue)
        }
    }
}
