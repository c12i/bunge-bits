## Database Migrations & Development

### 1. Install `sqlx-cli`

See the [sqlx-cli documentation](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query) for installation instructions.

---

### 2. Start a Development Database

Start the dev database using Docker Compose:

```
docker compose -f docker-compose.dev.yml up -d
```

---

### 3. Set Up Environment Variables

Copy `.env.development` to `.env` in the `stream_datastore` directory:

```
cp .env.development .env
```

---

### 4. Add a Migration

While in the `stream_datastore` directory, add a new migration:

```
sqlx migrate add <migration_name>
```

---

### 5. Run Migrations

Migrations run automatically when `DataStore::init` is called.

To run migrations manually via the CLI:

```
sqlx migrate run
```

---

### 6. Testing Schema Changes

For extra confidence in your DB/schema changes, write a test.  
See `test_bulk_insert_and_check_existing_streams_works` for an example of how to structure these tests.
