//! MySQLAdapter — MySQL 8.0 InnoDB database driver.
use super::{DatabaseAdapter, AdapterKind};

pub struct MySQLAdapter {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub user: String,
    connected: bool,
}

impl MySQLAdapter {
    pub fn new(host: &str, port: u16, db_name: &str, user: &str) -> Self {
        MySQLAdapter { host: host.into(), port, db_name: db_name.into(), user: user.into(), connected: false }
    }
}

impl DatabaseAdapter for MySQLAdapter {
    fn connect(&mut self) -> bool {
        eprintln!("[MySQLAdapter] TCP → {}:{} db={} user={}", self.host, self.port, self.db_name, self.user);
        self.connected = true;
        true
    }
    fn disconnect(&mut self) -> bool { self.connected = false; true }
    fn execute(&self, sql: &str) -> Result<Vec<String>, String> {
        if !self.connected { return Err("Not connected".into()); }
        Ok(vec![format!("[MySQL OK] {}", sql)])
    }
    fn driver_name(&self) -> &str { "MySQL 8.0" }
    fn kind(&self) -> AdapterKind { AdapterKind::MySQL }
    fn is_connected(&self) -> bool { self.connected }
}
