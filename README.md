# Hotel Booking API

A RESTful API built with Rust using Rocket and SeaORM for managing hotel bookings. This API provides endpoints for managing hotels, rooms, guests, and bookings.

## ⚡️ Tech Stack

### 🚀 Framework: [Rocket](https://rocket.rs/) v0.5.1
- Type-safe web framework for Rust
- Built-in JSON support
- Robust request handling and routing
- Zero-cost abstractions

### 🌊 ORM: [SeaORM](https://www.sea-ql.org/SeaORM/) v1.1.0
- Async & Dynamic
- Database-agnostic design
- Type-safe query building
- Schema management with migrations

## Features

- 🏨 Hotel management (CRUD operations)
- 🛏️ Room management with availability tracking
- 👥 Guest management
- 📅 Booking management with status tracking
- 📚 OpenAPI/Swagger documentation
- 🔒 Type-safe database operations with SeaORM
- 🚀 Built with Rust's Rocket framework

## Prerequisites

- Rust (latest stable version)
- PostgreSQL 13 or higher
- Docker & Docker Compose (optional, for containerized database)

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/yourusername/hotel_booking
cd hotel_booking
```

2. Setup the database:
   
   Using Docker Compose:
```bash
docker-compose up -d
```

   Or use your existing PostgreSQL instance and update the `.env` file accordingly.

3. Create a `.env` file in the project root:
```env
DB_USER=postgres
DB_PASS=seaorm_123
DB_NAME=seaorm_db
DB_PORT=5432

PORT=8000
```

4. Run the migrations:
```bash
cargo run --bin migration
```

5. Start the server:
```bash
cargo run
```

The API will be available at `http://localhost:8000/api/v1`

## API Documentation

Once the server is running, you can access the OpenAPI/Swagger documentation at:
`http://localhost:8000/swagger-ui/`

### Available Endpoints

#### Hotels
- `GET /api/v1/hotels` - List all hotels
- `GET /api/v1/hotels/{id}` - Get a specific hotel
- `POST /api/v1/hotels` - Create a new hotel
- `PUT /api/v1/hotels/{id}` - Update a hotel
- `DELETE /api/v1/hotels/{id}` - Delete a hotel

#### Rooms
- `GET /api/v1/rooms` - List all rooms
- `GET /api/v1/rooms/{id}` - Get a specific room
- `POST /api/v1/rooms` - Create a new room
- `PUT /api/v1/rooms/{id}` - Update a room
- `DELETE /api/v1/rooms/{id}` - Delete a room
- `GET /api/v1/hotels/{hotel_id}/rooms` - Get rooms for a specific hotel

#### Guests
- `GET /api/v1/guests` - List all guests
- `GET /api/v1/guests/{id}` - Get a specific guest
- `POST /api/v1/guests` - Create a new guest
- `PUT /api/v1/guests/{id}` - Update a guest
- `DELETE /api/v1/guests/{id}` - Delete a guest

#### Bookings
- `GET /api/v1/bookings` - List all bookings
- `GET /api/v1/bookings/{id}` - Get a specific booking
- `POST /api/v1/bookings` - Create a new booking
- `PUT /api/v1/bookings/{id}` - Update a booking
- `DELETE /api/v1/bookings/{id}` - Delete a booking
- `GET /api/v1/guests/{guest_id}/bookings` - Get bookings for a specific guest
- `GET /api/v1/rooms/{room_id}/bookings` - Get bookings for a specific room

## Project Structure

```
.
├── Cargo.toml
├── docker-compose.yml
└── src
    ├── config/          # Configuration management
    ├── models/          # Database models
    ├── routes/          # API endpoints
    ├── schemas/         # Request/Response schemas
    ├── services/        # Business logic
    └── main.rs          # Application entry point
```

## Database Schema

The application uses PostgreSQL with the following main entities:
- `hotels` - Hotel information
- `rooms` - Room details and availability
- `guests` - Guest information
- `bookings` - Booking records with status tracking

## Development

### Building for Production
```bash
cargo build --release
```

### Database Migrations

Create a new migration:
```bash
sea-orm-cli migrate generate create_new_table
```


## Built With
- [Rust](https://www.rust-lang.org/) - The programming language
- [Rocket](https://rocket.rs/) - Web framework
- [SeaORM](https://www.sea-ql.org/SeaORM/) - ORM framework
- [PostgreSQL](https://www.postgresql.org/) - Database
- [Utoipa](https://github.com/juhaku/utoipa) - OpenAPI documentation