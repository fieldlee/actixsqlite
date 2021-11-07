use crate::Pool;
use crate::model::{JsonProduct, JsonProductByUp, PostProduct, Product};
use crate::wshandler;

use actix_web::{HttpRequest,HttpResponse,Error, get,post,web};
use diesel::dsl::insert_into;
use diesel::{RunQueryDsl,delete,update};
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


#[get("/get_products/{start}/{count}")]
pub async fn get_products(
    pool:web::Data<Pool>,
    path:web::Path<Vec<String>>
) -> Result<HttpResponse,Error>{
    Ok(get_all_products(pool,path)
        .await
        .map(move|product|HttpResponse::Created().json(product))
        .map_err(|_|HttpResponse::InternalServerError())?
    )
}

async fn get_all_products(
    pool:web::Data<Pool>,
    path:web::Path<Vec<String>>
)->Result<Vec<Product>,diesel::result::Error>{
    use crate::schema::products::dsl::*;
    let db_connect = pool.get().unwrap();
    let start = &path.0[0];
    let count = &path.0[1];
    let i_start = start.parse().unwrap();
    let i_count = count.parse().unwrap();
    products.offset(i_start).limit(i_count).load::<Product>(&db_connect)
}

#[get("/del_product/{id}")]
pub async fn del_product(
    pool:web::Data<Pool>,
    path:web::Path<String>
) -> Result<HttpResponse,Error>{
    Ok(del_product_byid(pool,path)
        .await
        .map(move|product|HttpResponse::Created().json(product))
        .map_err(|_|HttpResponse::InternalServerError())?
    )
}

async fn del_product_byid(
    pool:web::Data<Pool>,
    path:web::Path<String>
)->Result<usize,diesel::result::Error>{
    use crate::schema::products::dsl::*;
    let db_connect = pool.get().unwrap();
    let by_id = &path.0;
    let i_by_id:i32 = by_id.parse().unwrap();
    let result = delete(products.filter(id.eq(i_by_id))).execute(&db_connect)?;
    Ok(result)
}



#[post("/update_product")]
pub async fn update_product(
    pool:web::Data<Pool>,
    item:web::Json<JsonProductByUp>
) -> Result<HttpResponse,Error>{
    Ok(update_product_byid(pool,item)
        .await
        .map(move|product|HttpResponse::Created().json(product))
        .map_err(|_|HttpResponse::InternalServerError())?
    )
}

async fn update_product_byid(
    pool:web::Data<Pool>,
    item:web::Json<JsonProductByUp>
)->Result<usize,diesel::result::Error>{
    use crate::schema::products::dsl::*;
    let db_connect = pool.get().unwrap();
    let sizeup = update(products.filter(id.eq(&item.id)))
    .set((
        name.eq(&item.name),
        title.eq(&item.title),
        created_at.eq(&format!("{}",chrono::Local::now().naive_local()))
    ))
    .execute(&db_connect).expect("update erorr");
    Ok(sizeup)

}

pub async fn home() -> Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().body("ok"))
}

pub async fn ws_handle(
    req: HttpRequest,
	stream: web::Payload,) -> Result<HttpResponse,Error>{
    let resp = actix_web_actors::ws::start(wshandler::WsCon{nick:"".to_string()}, &req, stream);
    resp
}
