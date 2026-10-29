# Mantra REST Application (`my_rest_app`)

A microservice providing RESTful services for a `users` table built on an interchangeable Object-Relational Mapping (ORM) architecture in Mantra.

---

## Architecture Overview

```text
       ┌────────────────────────┐
       │   HTTP Client Requests │
       └───────────┬────────────┘
                   │
                   ▼
       ┌────────────────────────┐
       │   UserRestController   │  (GET, POST, PUT, DELETE /api/v1/users)
       └───────────┬────────────┘
                   │
                   ▼
       ┌────────────────────────┐
       │        UserORM         │  (CRUD Repository: create, find, update, delete)
       └───────────┬────────────┘
                   │
                   ▼
       ┌────────────────────────┐
       │ DatabaseAdapter (IFace)│  (Pluggable Database Contract)
       └─────┬────────────┬─────┘
             │            │
             ▼            ▼
     ┌──────────────┐  ┌──────────────────┐
     │ MySQLAdapter │  │  PostgresAdapter │  (Swappable via .env: DB_TYPE)
     └──────────────┘  └──────────────────┘
```

---

## 1. Environment Configuration (`.env`)

The database connection is configured via `.env`:

```ini
# Database Connection
DB_TYPE=mysql
DB_HOST=127.0.0.1
DB_PORT=3306
DB_NAME=users_db
DB_USER=root
DB_PASSWORD=secret_password_123

# Server Settings
SERVER_PORT=8080
API_VERSION=v1
APP_NAME=Mantra User REST Service
```

---

## 2. Database Interchangeability (Pluggable Adapters)

To switch the database from **MySQL** to **PostgreSQL** or another engine, you only need to change `DB_TYPE`:

```ini
# Change in .env:
DB_TYPE=postgres
```

The `DatabaseFactory` dynamically instantiates the corresponding adapter, and the `UserORM` repository continues executing operations without changing any application code.

---

## 3. REST API Endpoints

| Method | Endpoint | Description | Status Code |
|:---|:---|:---|:---|
| `GET` | `/api/v1/users` | List all users | `200 OK` |
| `GET` | `/api/v1/users/:id` | Fetch user by ID | `200 OK` or `404 Not Found` |
| `POST` | `/api/v1/users` | Create new user | `201 Created` |
| `PUT` | `/api/v1/users/:id` | Update user details | `200 OK` or `404 Not Found` |
| `DELETE` | `/api/v1/users/:id` | Delete user | `200 OK` or `404 Not Found` |

---

## 4. Running the Application

### Development Run (VM Mode)
```bash
cd my_rest_app
mantra run
```

### Run Tests
```bash
mantra test
```
