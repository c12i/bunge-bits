fn main() {
    // context on why we need this: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#triggering-recompilation-on-migration-changes
    println!("cargo:rerun-if-changed=migrations");
}
