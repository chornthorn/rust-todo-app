# Rust Todo API with Actix-Web

## Description

This is a simple todo API built with Rust and Actix-Web. It uses MySQL database to store the todos.

## Prerequisites

Before you begin, ensure you have met the following requirements:

* You have installed the latest version of [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Installing <Project Name>

To install <Project Name>, follow these steps:

1. Clone the repository:
```sh
git clone https://github.com/chornthorn/rust-todo-app.git
```
2. Navigate to the project directory:
```sh
cd rust-todo-app
```

## Setting Up the Database

Before running the application, you need to set up the database. This project uses `sqlx`, which supports multiple database systems. The instructions below are for MySQL.

1. Install MySQL if you haven't already. You can download it from [here](https://dev.mysql.com/downloads/installer/).

2. Create a new MySQL database for the project.

3. Copy the `.env.example` file to a new file named `.env`:

```sh
cp .env.example .env
```

4. Open the `.env` file and set the `DATABASE_URL` environment variable to the connection string for your database. For example, it might look like this:

```sh
DATABASE_URL=mysql://username:password@localhost/database_name
```

Replace `username`, `password`, and `database_name` with your actual MySQL username, password, and the name of the database you created.

## Running Migrations

This project uses `sqlx` for database migrations. To run migrations, follow these steps:

1. Install the `sqlx-cli` tool if you haven't already:

```sh
brew install sqlx-cli
```


2. Run the migrations:

```sh
sqlx migrate run
```
or using cargo:

```sh
cargo sqlx migrate run
```

This will apply all migrations in the `migrations` directory to your database.

## Using <Project Name>

To use <Project Name>, follow these steps:

1. Build the project:
```sh
cargo build --release
```
2. Run the project:
```sh
cargo run
```

## Running Tests

To run tests, use the following command:

```sh
cargo test
```

## Contributing to Rust Todo API with Actix-Web

To contribute to Rust Todo API with Actix-Web, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`
4. Push to the original branch: `git push origin <project_name>/<location>`
5. Create the pull request.

## Contact

If you want to contact me you can reach me at `thorn.c@khodedev.com`.

## License

This project uses the following license: [MIT](<link>).