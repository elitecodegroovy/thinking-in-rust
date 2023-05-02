Here's a complete example of CRUD operations using the Diesel ORM framework in Rust. Before you start, make sure you have Diesel CLI installed and set up a new Diesel project with a PostgreSQL database.

1. Add Diesel and dotenv dependencies to your `Cargo.toml`:

```toml
[dependencies]
diesel = { version = "1.4.8", features = ["postgres", "chrono"] }
dotenv = "0.15.0"
```

2. Create a `.env` file in your project root with your database URL:

```
DATABASE_URL=postgres://username:password@localhost/db_name
```

3. Create a `migrations` folder and a subfolder named `0001_create_users` with two files: `up.sql` and `down.sql`.

`up.sql`:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL
);
```

`down.sql`:

```sql
DROP TABLE users;
```

4. Run the migration:

```bash
diesel migration run
```

5. Create a `src/schema.rs` file and run:

```bash
diesel print-schema > src/schema.rs
```

6. Now, let's write the Rust code for CRUD operations in `src/main.rs`:

```rust
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{User, NewUser};

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).expect("Error connecting to database");

    // Create
    let new_user = NewUser {
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    };
    let user = create_user(&conn, new_user);
    println!("Created user: {:?}", user);

    // Read
    let all_users = get_all_users(&conn);
    println!("All users: {:?}", all_users);

    // Update
    let updated_user = update_user_name(&conn, user.id, "Jane Doe".to_string());
    println!("Updated user: {:?}", updated_user);

    // Delete
    let deleted_user = delete_user(&conn, updated_user.id);
    println!("Deleted user: {:?}", deleted_user);
}

fn create_user(conn: &PgConnection, new_user: NewUser) -> User {
    use schema::users;

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error creating user")
}

fn get_all_users(conn: &PgConnection) -> Vec<User> {
    use schema::users::dsl::*;

    users.load::<User>(conn).expect("Error loading users")
}

fn update_user_name(conn: &PgConnection, user_id: i32, new_name: String) -> User {
    use schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user_id)))
        .set(name.eq(new_name))
        .get_result(conn)
        .expect("Error updating user name")
}

fn delete_user(conn: &PgConnection, user_id: i32) -> User {
    use schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id)))
        .get_result(conn)
        .expect("Error deleting user")
}
```

7. Finally, create the `src/models.rs` file:

```rust
use super::schema::users;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
```

Now you have a complete example of CRUD operations using the Diesel ORM framework in Rust.