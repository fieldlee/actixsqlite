use crate::Pool;
use crate::model::{JsonProduct,PostProduct,Product};

use actix_web::{HttpResponse,Error, post,web};
use diesel::dsl::insert_into;
use diesel::RunQueryDsl;
use diesel::prelude::*;

#[post("/add_product")]
pub async fn add_product(
    p:web::Data<Pool>,
    item:web::Json<JsonProduct>
) -> Result<HttpResponse,Error>{
    Ok(add_single_product(p,item)
        .await
        .map(move|product|HttpResponse::Created().json(product))
        .map_err(|_|HttpResponse::InternalServerError())?
    )
}

async fn add_single_product(
    pool:web::Data<Pool>,
    item:web::Json<JsonProduct>
)->Result<Product,diesel::result::Error>{
    use crate::schema::products::dsl::*;
    let db_connect = pool.get().unwrap();
    match products
    .filter(name.eq(&item.name))
    .first::<Product>(&db_connect){
        Ok(result)=>Ok(result),
        Err(_)=>{
            let new_product = PostProduct{
                name : &item.name,
                title : &item.title,
                created_at:&format!("{}",chrono::Local::now().naive_local())
            };
            insert_into(products)
            .values(&new_product)
            .execute( &db_connect)
            .expect("Error insert product");

            let result = products
            .order(id.desc())
            .first(&db_connect)
            .expect("insert into error");
            Ok(result)
        }
    }
}