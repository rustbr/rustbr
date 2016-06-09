extern crate uuid;
extern crate pwhash;
extern crate chrono;

use libraries::db::ArcPool;
use self::uuid::Uuid;
use self::pwhash::sha512_crypt;
use self::chrono::*;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    password: String,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>
}

impl User {
    pub fn new(pool: ArcPool,
               email: String,
               first_name: String,
               last_name: String,
               password: String)
               -> User {
        let id = Uuid::new_v4();
        let hash = sha512_crypt::hash(&password).unwrap();
        let created_at: DateTime<UTC> = UTC::now();
        let updated_at: DateTime<UTC> = UTC::now();

        pool.get()
            .unwrap()
            .execute("INSERT INTO users (id, email, first_name, last_name, password, created_at, \
                      updated_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
                     &[&id, &email, &first_name, &last_name, &hash, &created_at, &updated_at])
            .unwrap();

        User {
            id: id,
            email: email,
            first_name: first_name,
            last_name: last_name,
            password: hash,
            created_at: created_at,
            updated_at: updated_at
        }
    }

    pub fn find_by_email(pool: ArcPool, email: String) -> User {
        let conn = pool.get().unwrap();
        let stmt = conn.query("SELECT * from users where email = $1 LIMIT 1", &[&email]).unwrap();
        let row = stmt.iter().next().unwrap();

        User {
            id: row.get("id"),
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            password: row.get("password"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        }
    }

    pub fn check_password(&self, password: String) -> bool {
        sha512_crypt::verify(&password, &self.password)
    }
}
