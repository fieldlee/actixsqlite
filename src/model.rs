use serde::{Deserialize,Serialize};
use crate::schema::*;

#[derive(Debug,Serialize,Deserialize,Queryable)]
pub struct Product{
    pub id:i32,
    pub name :String,
    pub title :String,
    pub created_at:String,
}

#[derive(Debug,Insertable)]
#[table_name = "products"]
pub struct PostProduct<'a>{
    pub name : &'a str,
    pub title : &'a str,
    pub created_at:&'a str,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct JsonProduct{
    pub name : String,
    pub title : String,
}