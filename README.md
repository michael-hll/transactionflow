# TransactionFlow API

A Rust-based backend API service for managing financial transactions.

## Features

- User authentication (JWT)
- Category management
- Transaction tracking

## Technologies

- Actix Web
- PostgreSQL with SQLx
- JWT authentication

## Setup

1. Install Rust and PostgreSQL
2. Clone the repository
3. Create a `.env` file with your database URL
4. Run with `cargo run`

## Database Schema

### ER Diagram

### Tables

#### users

- `id`: BIGSERIAL PRIMARY KEY
- `email`: VARCHAR(255) NOT NULL UNIQUE
- `password`: VARCHAR(255) NOT NULL
- `first_name`: VARCHAR(100) NOT NULL
- `last_name`: VARCHAR(100) NOT NULL
- `balance`: DECIMAL(15, 2) DEFAULT 0.00 NOT NULL
- `created_at`: TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
- `updated_at`: TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP

#### categories

- `id`: BIGSERIAL PRIMARY KEY
- `user_id`: BIGINT NOT NULL (Foreign key to users.id)
- `name`: VARCHAR(100) NOT NULL
- `description`: TEXT
- `balance`: DECIMAL(15, 2) DEFAULT 0.00 NOT NULL
- `created_at`: TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
- `updated_at`: TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP

#### transactions

- `id`: BIGSERIAL PRIMARY KEY
- `user_id`: BIGINT NOT NULL (Foreign key to users.id)
- `category_id`: BIGINT NOT NULL (Foreign key to categories.id)
- `type`: VARCHAR(100) NOT NULL
- `amount`: DECIMAL(15, 2) NOT NULL
- `memo`: TEXT
- `description`: TEXT
- `created_at`: TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
- `updated_at`: TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP

## API Reference

### Authentication

#### Sign Up

- **POST** `/auth/sign-up`
- Body: `{ "email": "user@example.com", "password": "secure123", "first_name": "John", "last_name": "Doe" }`
- Returns: User details with JWT token

#### Sign In

- **POST** `/auth/sign-in`
- Body: `{ "email": "user@example.com", "password": "secure123" }`
- Returns: User details with JWT token

### User Profile

#### Get Profile

- **GET** `/users/profile`
- Headers: `Authorization: Bearer <token>`
- Returns: Current user profile

#### Update Profile

- **PUT** `/users/profile`
- Headers: `Authorization: Bearer <token>`
- Body: `{ "first_name": "John", "last_name": "Doe" }`
- Returns: Updated user profile

### Categories

#### List Categories

- **GET** `/categories`
- Headers: `Authorization: Bearer <token>`
- Returns: List of user's categories

#### Get Category

- **GET** `/categories/{id}`
- Headers: `Authorization: Bearer <token>`
- Returns: Category details

#### Create Category

- **POST** `/categories`
- Headers: `Authorization: Bearer <token>`
- Body: `{ "name": "Groceries", "description": "Food and household items" }`
- Returns: Created category

#### Update Category

- **PUT** `/categories/{id}`
- Headers: `Authorization: Bearer <token>`
- Body: `{ "name": "Groceries", "description": "Food and household items" }`
- Returns: Updated category

#### Delete Category

- **DELETE** `/categories/{id}`
- Headers: `Authorization: Bearer <token>`
- Returns: Success status

### Transactions

#### List Transactions

- **GET** `/transactions`
- Headers: `Authorization: Bearer <token>`
- Query Params: `?category_id=1&type=expense` (optional)
- Returns: List of transactions

#### Get Transaction

- **GET** `/transactions/{id}`
- Headers: `Authorization: Bearer <token>`
- Returns: Transaction details

#### Create Transaction

- **POST** `/transactions`
- Headers: `Authorization: Bearer <token>`
- Body: `{ "category_id": 1, "type": "expense", "amount": 24.99, "memo": "Grocery shopping" }`
- Returns: Created transaction

#### Update Transaction

- **PUT** `/transactions/{id}`
- Headers: `Authorization: Bearer <token>`
- Body: `{ "category_id": 1, "amount": 24.99, "memo": "Updated description" }`
- Returns: Updated transaction

#### Delete Transaction

- **DELETE** `/transactions/{id}`
- Headers: `Authorization: Bearer <token>`
- Returns: Success status
