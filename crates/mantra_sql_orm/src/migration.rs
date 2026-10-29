//! Migration — Schema migration system for Mantra ORM. NEW in v0.2.0.

/// Defines a single column in a CREATE TABLE statement.
#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub col_type: String,
    pub not_null: bool,
    pub primary_key: bool,
    pub auto_increment: bool,
}

impl ColumnDef {
    pub fn new(name: &str, col_type: &str) -> Self {
        ColumnDef { name: name.into(), col_type: col_type.into(), not_null: false, primary_key: false, auto_increment: false }
    }
    pub fn not_null(mut self) -> Self  { self.not_null = true; self }
    pub fn primary(mut self) -> Self   { self.primary_key = true; self }
    pub fn auto_inc(mut self) -> Self  { self.auto_increment = true; self }
}

/// Fluent schema builder for CREATE TABLE DDL.
pub struct SchemaBuilder {
    table: String,
    columns: Vec<ColumnDef>,
}

impl SchemaBuilder {
    pub fn create_table(table: &str) -> Self {
        SchemaBuilder { table: table.to_string(), columns: vec![] }
    }
    pub fn column(mut self, def: ColumnDef) -> Self { self.columns.push(def); self }
    pub fn build_ddl(&self) -> String {
        let cols: Vec<String> = self.columns.iter().map(|c| {
            let mut s = format!("  {} {}", c.name, c.col_type);
            if c.primary_key    { s.push_str(" PRIMARY KEY"); }
            if c.auto_increment { s.push_str(" AUTOINCREMENT"); }
            if c.not_null       { s.push_str(" NOT NULL"); }
            s
        }).collect();
        format!("CREATE TABLE IF NOT EXISTS {} (\n{}\n);", self.table, cols.join(",\n"))
    }
}

/// Represents a single migration with an up (apply) and down (rollback) script.
pub struct Migration {
    pub version: u32,
    pub name: String,
    pub up_sql: String,
    pub down_sql: String,
}

impl Migration {
    pub fn new(version: u32, name: &str, up: &str, down: &str) -> Self {
        Migration { version, name: name.into(), up_sql: up.into(), down_sql: down.into() }
    }
}

/// Runs pending migrations against a database adapter.
pub struct MigrationRunner {
    migrations: Vec<Migration>,
}

impl MigrationRunner {
    pub fn new() -> Self { MigrationRunner { migrations: vec![] } }
    pub fn register(mut self, m: Migration) -> Self { self.migrations.push(m); self }
    pub fn run_all(&self, adapter: &dyn crate::adapters::DatabaseAdapter) -> Vec<String> {
        let mut results = vec![];
        for m in &self.migrations {
            let result = adapter.execute(&m.up_sql);
            match result {
                Ok(_)  => results.push(format!("[OK] Migration {} — {}", m.version, m.name)),
                Err(e) => results.push(format!("[ERR] Migration {} failed: {}", m.version, e)),
            }
        }
        results
    }
}
