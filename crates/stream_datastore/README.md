## Adding a migration:

### Install sqlx cli

https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query

### Running a dev DB via docker-compose

```
docker compose -f docker-compose.dev.yml up -d
```

### Create a `.env` file

- Copy the contents of `.env.development` to a `.env` file (still in the stream_datastore directory)

### Adding the migration (while in the `stream_datastore` directory)

```
sqlx migrate add <migration_name>
```

### Running the migrations

- The migrations automatically run when `DataStore::init` runs

- You can also test if your migrations work via the cli

```
sqlx migrate run
```

### Adding tests

- If you want additional confidence that your db / schema changes are sound, please write a test.
- Check `test_bulk_insert_and_check_existing_streams_works` to see how to write / structure the test.
