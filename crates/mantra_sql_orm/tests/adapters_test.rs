use mantra_sql_orm::*;

#[test]
fn test_oracle_adapter_procedures_and_cursors() {
    let mut adapter = OracleAdapter::new("oracle://localhost:1521/XE");
    assert!(adapter.connect());
    assert!(adapter.is_connected());

    // Procedure
    let params = vec![
        ProcedureParam::input("emp_id", FieldValue::Int(1001)),
        ProcedureParam::output("emp_name", "VARCHAR2"),
    ];
    let res = adapter.execute_procedure("emp_pkg.get_employee", &params).unwrap();
    assert_eq!(res.status, 0);
    assert!(res.output_params.contains_key("emp_name"));

    // Cursor
    let mut cursor = adapter.open_cursor("emp_cursor", "SELECT * FROM employees").unwrap();
    assert!(cursor.is_open);
    let row = cursor.fetch_next().unwrap().unwrap();
    assert_eq!(row.get_text("CURSOR_NAME"), Some("emp_cursor"));
    assert!(cursor.close().is_ok());

    // View
    let create_view_sql = adapter.create_view("emp_view", "SELECT * FROM emp", true).unwrap();
    assert!(create_view_sql.contains("CREATE MATERIALIZED VIEW"));
    let refresh_sql = adapter.refresh_materialized_view("emp_view").unwrap();
    assert!(refresh_sql.contains("DBMS_MVIEW.REFRESH"));

    // Function
    let fn_res = adapter.execute_scalar_function("calc_bonus", &[FieldValue::Int(50000)]).unwrap();
    assert!(fn_res.to_string().contains("ORACLE_FN_RESULT"));
}

#[test]
fn test_mssql_adapter_procedures_and_views() {
    let mut adapter = MSSQLAdapter::new("Server=localhost;Database=MantraDB;");
    assert!(adapter.connect());
    assert!(adapter.is_connected());

    // Stored Procedure
    let params = vec![
        ProcedureParam::input("CustomerId", FieldValue::Int(42)),
        ProcedureParam::output("TotalOrders", "INT"),
    ];
    let res = adapter.execute_procedure("sp_GetCustomerOrders", &params).unwrap();
    assert_eq!(res.status, 0);
    assert_eq!(res.output_params.get("TotalOrders"), Some(&FieldValue::Text("MSSQL_OUT_VAL".to_string())));

    // Cursor
    let mut cursor = adapter.open_cursor("cust_cursor", "SELECT * FROM Customers").unwrap();
    assert_eq!(cursor.direction, CursorDirection::Scrollable);
    let row = cursor.fetch_next().unwrap().unwrap();
    assert_eq!(row.get_text("CURSOR_NAME"), Some("cust_cursor"));

    // View
    let view_sql = adapter.create_view("v_Orders", "SELECT * FROM Orders", false).unwrap();
    assert!(view_sql.contains("CREATE OR ALTER VIEW"));

    // Function
    let fn_res = adapter.execute_scalar_function("fn_GetTaxRate", &[FieldValue::Text("CA".to_string())]).unwrap();
    assert!(fn_res.to_string().contains("dbo.fn_GetTaxRate"));
}

#[test]
fn test_mongodb_adapter_document_operations_and_views() {
    let mut adapter = MongoDBAdapter::new("mongodb://localhost:27017", "mantra_app");
    assert!(adapter.connect());
    assert!(adapter.is_connected());

    // MongoDB system function / script
    let params = vec![
        ProcedureParam::input("filter", FieldValue::Text("active: true".to_string())),
    ];
    let res = adapter.execute_procedure("compute_user_stats", &params).unwrap();
    assert_eq!(res.status, 1);
    assert_eq!(res.output_params.get("status"), Some(&FieldValue::Text("executed".to_string())));

    // Cursor
    let mut cursor = adapter.open_cursor("doc_cursor", "{ status: 'A' }").unwrap();
    let doc = cursor.fetch_next().unwrap().unwrap();
    assert_eq!(doc.get_text("_id"), Some("doc_cursor_1"));

    // View
    let view_cmd = adapter.create_view("active_users_view", "[{ $match: { active: true } }]", false).unwrap();
    assert!(view_cmd.contains("db.createView"));

    // Function
    let fn_res = adapter.execute_scalar_function("calculateScore", &[FieldValue::Int(95)]).unwrap();
    assert!(fn_res.to_string().contains("db.eval"));
}

#[test]
fn test_generic_db_adapter() {
    let mut adapter = GenericDbAdapter::new("snowflake", "snowflake://account.snowflakecomputing.com");
    assert!(adapter.connect());
    assert_eq!(adapter.driver_name(), "snowflake");

    let params = vec![ProcedureParam::input("tenant", FieldValue::Text("t_1".to_string()))];
    let res = adapter.execute_procedure("sp_sync_tenant", &params).unwrap();
    assert_eq!(res.status, 0);

    let mut cursor = adapter.open_cursor("c1", "SELECT 1").unwrap();
    let rows = cursor.fetch_many(10).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(cursor.close().is_ok());
    assert!(cursor.fetch_next().is_err());
}

