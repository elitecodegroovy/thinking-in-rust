
## Add submodule

```
cargo new handle-errors --lib
```

## project structure layout

Here's a suggested project layout for a Warp-based Rust web application. This layout separates concerns and organizes the code into modules for better maintainability.

```
my_warp_project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── greeting.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── greeting.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── greeting.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── filters.rs
```

Here's a brief explanation of each file:

- `Cargo.toml`: Contains the project's dependencies and metadata.
- `src/main.rs`: The main entry point of the application, responsible for setting up the server and combining the routes.
- `src/handlers/mod.rs`: Exports the handlers module.
- `src/handlers/greeting.rs`: Contains the request handlers for the greeting-related endpoints.
- `src/models/mod.rs`: Exports the models module.
- `src/models/greeting.rs`: Contains the `Greeting` struct and any related logic.
- `src/routes/mod.rs`: Exports the routes module.
- `src/routes/greeting.rs`: Defines the greeting-related routes and their corresponding handlers.
- `src/utils/mod.rs`: Exports the utils module.
- `src/utils/filters.rs`: Contains utility filters, such as shared state filters.

With this layout, you can organize your code in a modular way, making it easier to maintain and expand your application. Remember to update the `mod.rs` files to export the appropriate modules and structs.

```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs

```