
# mysql_helper
Make mysql to struct simple with macro

# Example

`// Create a strcut which the feilds same with mysql-table feilds`\
`#[derive(ModelHelper)]`\
`struct CustomStrcut{`\
`    pub c_id : Option<i64>,`\
`    pub c_name : Option<String>,`\
`}`

`// conn is mysql connection`\
`let conn = ...;`\
`// ModelHelper will helper you create 'mysql_to_vo' method`\
`let v:Vec<CustomStrcut> = conn.query_map("select * from xxx",|row:Row|CustomStrcut::mysql_to_vo(row));`\
Create connection,see [mysql](https://crates.io/crates/mysql)