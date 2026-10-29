//! QueryBuilder — Fluent SQL query construction for Mantra models.

/// Ordering direction for ORDER BY clauses.
#[derive(Debug, Clone)]
pub enum OrderBy {
    Asc(String),
    Desc(String),
}

/// A single WHERE predicate.
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub column: String,
    pub op: String,      // "=", "!=", ">", "<", ">=", "<="
    pub value: String,
}

/// Fluent query builder for constructing SQL SELECT/INSERT/UPDATE/DELETE.
#[derive(Debug, Default)]
pub struct QueryBuilder {
    table: String,
    selects: Vec<String>,
    wheres: Vec<WhereClause>,
    order: Option<OrderBy>,
    limit_val: Option<usize>,
    offset_val: Option<usize>,
}

impl QueryBuilder {
    /// Start building a query against the given table.
    pub fn from(table: &str) -> Self {
        QueryBuilder { table: table.to_string(), selects: vec!["*".into()], ..Default::default() }
    }

    /// Select specific columns.
    pub fn select(mut self, columns: &[&str]) -> Self {
        self.selects = columns.iter().map(|c| c.to_string()).collect();
        self
    }

    /// Add a WHERE column = value condition.
    pub fn where_eq(mut self, column: &str, value: &str) -> Self {
        self.wheres.push(WhereClause { column: column.into(), op: "=".into(), value: value.into() });
        self
    }

    /// Add a WHERE column != value condition.
    pub fn where_ne(mut self, column: &str, value: &str) -> Self {
        self.wheres.push(WhereClause { column: column.into(), op: "!=".into(), value: value.into() });
        self
    }

    /// Add a WHERE column > value condition.
    pub fn where_gt(mut self, column: &str, value: &str) -> Self {
        self.wheres.push(WhereClause { column: column.into(), op: ">".into(), value: value.into() });
        self
    }

    /// ORDER BY column ASC.
    pub fn order_asc(mut self, column: &str) -> Self {
        self.order = Some(OrderBy::Asc(column.to_string()));
        self
    }

    /// ORDER BY column DESC.
    pub fn order_desc(mut self, column: &str) -> Self {
        self.order = Some(OrderBy::Desc(column.to_string()));
        self
    }

    /// Limit result count.
    pub fn limit(mut self, n: usize) -> Self { self.limit_val = Some(n); self }

    /// Skip the first n rows.
    pub fn offset(mut self, n: usize) -> Self { self.offset_val = Some(n); self }

    /// Build the final SQL SELECT string.
    pub fn build_select(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}", self.selects.join(", "), self.table);
        if !self.wheres.is_empty() {
            let predicates: Vec<String> = self.wheres.iter()
                .map(|w| format!("{} {} '{}'", w.column, w.op, w.value))
                .collect();
            sql.push_str(&format!(" WHERE {}", predicates.join(" AND ")));
        }
        if let Some(ref ord) = self.order {
            match ord {
                OrderBy::Asc(col)  => sql.push_str(&format!(" ORDER BY {} ASC", col)),
                OrderBy::Desc(col) => sql.push_str(&format!(" ORDER BY {} DESC", col)),
            }
        }
        if let Some(l) = self.limit_val  { sql.push_str(&format!(" LIMIT {}", l)); }
        if let Some(o) = self.offset_val { sql.push_str(&format!(" OFFSET {}", o)); }
        sql
    }
}
