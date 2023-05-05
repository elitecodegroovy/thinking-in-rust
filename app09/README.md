# Rust - Build a CRUD API with SQLX and PostgreSQL

In this article, you'll learn how to build a CRUD API in Rust using SQLX, Actix-web, and PostgreSQL. Learning how to build a CRUD API as a developer will equip you with valuable skills for building robust, maintainable, and scalable applications.

![Rust - Build a CRUD API with SQLX and PostgreSQL](https://codevoweb.com/wp-content/uploads/2023/01/Rust-Build-a-CRUD-API-with-SQLX-and-PostgreSQL.webp)

## Topics Covered

- Run the Rust SQLX Project Locally
- Run the Rust SQLX API with a React.js App
- Setup the Rust Project
- Setup PostgreSQL and pgAdmin with Docker
- Create and Migrate the Database Queries
- Create the SQLX Database Model
- Create the Validation Schemas
- Create CRUD Route Functions
    - Fetch All Records
    - Add New Record
    - Retrieve a Single Record
    - Edit an Existing Record
    - Delete a Record
    - Merge the Route Functions
- Register the Routes and Add CORS
- Test the Rust CRUD API
    - Perform the CREATE Operation
    - Perform the UPDATE Operation
    - Perform the READ Operation
    - Perform the DELETE Operation


Read the entire article here: [https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/](https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/)

Once the installation is complete, run this command to start the server. Also, the cargo-watch tool will watch the src directory for changes and restart the server.

``` 
cargo watch -q -c -w src/ -x run
```

Run the command below to install the sqlx-cli binary:

```shell
cargo install sqlx-cli
```


Once the installation is done, run this command to create the reversible migration files. 
This will create a migrations folder that contains the up/down migration files in the root
directory. init is the name of the “up” and “down” scripts.

```shell
sqlx migrate add -r init
```



Open the “up” script and add the following SQL queries:

migrations/20230123154041_init.up.sql

Run this command to push the “up” migration script to the database:

sqlx migrate run


This command will execute the “down” migration script.

sqlx migrate revert

