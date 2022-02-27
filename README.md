# Drop Cases

## Getting Started

```bash
# Start Postgres
docker-compose up -d

# Run the migrations
diesel setup
diesel migration run

# Run the app
cargo run
```

## Example structure

- `<module>`
  - controller.rs
  - model.rs
  - repository.rs
  - service.rs
- infra
  - postgres/schema.rs
  - services/s3Service.rs
- shared

## ORMS

- [rbatis](https://github.com/rbatis/rbatis)
- [sqlx](https://github.com/launchbadge/sqlx)
- [sea-orm](https://github.com/SeaQL/sea-orm)

## Postgres

- [docs](https://www.postgresqltutorial.com/postgresql-getting-started/)
- [db admin docs](https://www.postgresqltutorial.com/postgresql-administration/)
