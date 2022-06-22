///
/// Make mysql to struct simple with macro
/// 
/// # Example
/// 
/// #[derive(ModelHelper)]
/// struct CustomStrcut{
///     pub c_id : Option<i64>,
///     pub c_name : Option<String>,
/// }
/// 
/// // conn is MySQL connection
/// let v:Vec<CustomStrcut> = conn.query_map("select * from xxx",|row:Row|CustomStrcut::mysql_to_vo(row));
/// 
use std::str::FromStr;

use syn::{__private::TokenStream, parse_macro_input, DeriveInput};
use syn::Data;
use syn::Fields;
#[proc_macro_derive(ModelHelper)]
pub fn derive_mysql_model_helper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("ModelHelper start");
    let ident = input.ident;
    let data = input.data;
    
    if let Data::Struct(a_strcut) = data {
        if let Fields::Named(feild_name) = a_strcut.fields {
            let named = feild_name.named;
            // println!("named:{:?}",named);
            let mut v = Vec::new();
            named.iter().for_each(|one|{
                if let Some(tmp_ident) = &one.ident {
                    v.push(format!("
                    {}:match row.index(\"{}\"){{
                        mysql::Value::NULL=>{{
                            None
                        }},
                        _other => {{
                            row.take(\"{}\")
                        }}
                    }},
                    ",tmp_ident,tmp_ident,tmp_ident))
                }
            });
            // println!("v = {:?}",v);
            let tmp_s = v.join("\r\n");
            let s = format!("
            impl {} {{ \r\n
                pub fn mysql_to_vo(row:mysql::Row)->Self{{ \r\n
                    let row = &mut row.clone(); \r\n
                    Self{{ \r\n
                        {}
                    }} \r\n
                }} \r\n
            }}",ident,tmp_s);
            // println!("sssss = {}",s);
            if let Ok(reuslt) = TokenStream::from_str(s.as_str()) {
                reuslt
            }else {
                unimplemented!();
            }
        }else {
            unimplemented!();
        }
    }else {
        unimplemented!();
    }
}
