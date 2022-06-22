
# mysql_helper
Make mysql to struct simple with macro

# Example



Create a strcut which the feilds same with mysql-table feilds

    use mysql_helper::ModelHelper;
    use std::ops::Index;
    #[derive(ModelHelper)]
    struct CustomStrcut{
        pub c_id : Option<i64>,
        pub c_name : Option<String>,
    }
    
Conn is mysql connection,Create connection,see [mysql](https://crates.io/crates/mysql).
ModelHelper will helper you create 'mysql_to_vo' method

    let v:Vec<CustomStrcut> = conn.query_map("select * from xxx",|row:Row|CustomStrcut::mysql_to_vo(row));
    
