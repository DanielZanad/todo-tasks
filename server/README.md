# Todo Tasks Server

This project is a Rust-based backend server for a Todo Tasks application. It follows a modular, layered architecture to ensure maintainability, scalability, and separation of concerns.

## Architecture Overview

### 1. **App Layer**

- **Entities:** Domain models (e.g., `User`) representing core business objects.
- **Repositories:** Abstractions for data access, allowing easy swapping of data sources.
- **Use Cases:** Business logic implementations (e.g., user registration).

### 2. **Infra Layer**

- **DB:** Database configuration and repository implementations using SQLx.
- **HTTP:** Controllers for handling HTTP requests and responses.

### 3. **Startup & Configuration**

- **Startup:** Application bootstrap logic, including server initialization.
- **Env:** Environment variable management for configuration.

### 4. **Migrations**

- SQL migration files for evolving the database schema.

### 5. **Signed URL API**

- A Node.js microservice for generating signed URLs (e.g., for file uploads).
- Located in `signed_url_api/` (ignore `node_modules`).

## Folder Structure

```
server/
├── migrations/                # SQL migration scripts
├── signed_url_api/            # Node.js microservice for signed URLs
│   ├── src/
│   ├── package.json
│   └── ...
├── src/
│   ├── app/
│   │   ├── entities/
│   │   ├── repositories/
│   │   └── use_cases/
│   ├── infra/
│   │   ├── db/
│   │   └── http/
│   ├── env.rs
│   ├── main.rs
│   ├── startup.rs
│   └── ...
├── .env
├── Cargo.toml
├── docker-compose.yml
└── ...
```

## Key Technologies

- **Rust**: Main backend language.
- **Actix Web**: Web framework for building fast and reliable web servers.
- **SQLx**: Async SQL toolkit for Rust.
- **Node.js**: Used in the `signed_url_api` microservice.

## Getting Started

1. **Configure environment variables** in `.env`.
2. **Run database migrations** in the `migrations/` folder.
3. **Start the Rust server**:

   ```sh
   cargo run
   ```

4. **(Optional) Start the signed URL API**:

   ```sh
   cd signed_url_api
   npm install
   npm start
   ```

## Notes

- The architecture is designed for extensibility and clear separation between domain, infrastructure, and business logic.

## API Endpoints

(Coming Soon: Detailed API endpoint documentation will be added here.)
