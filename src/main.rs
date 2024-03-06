
#[macro_use] extern crate rocket;

use std::mem;

use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Product {
 pub name: String,
}

async fn read(conn:&sqlx::PgPool) -> Result<Vec<Product>, sqlx::Error> {
    
    let q ="select t.\"Name\" as name from \"Products\" as p inner join \"ProductTranslations\" as t on p.\"Id\" = t.\"ProductId\" and t.\"Language\"='bg'";        

    let query = sqlx::query_as::<_,Product>(q);
    let prods =  query.fetch_all(conn).await?;
    
    Ok(prods)
}

#[get("/")]
async fn index() -> String {
    let pool = sqlx::PgPool::connect("postgres://root:root@localhost:5432/MilaTeneva")
    .await
    .expect("Failed to connect to database");

    let prods = read(&pool).await.unwrap();
    let mut res = String::new();
    for p in prods {
        res.push_str(&p.name);
        res.push_str("\n");
    }
    
    pool.close().await;
    mem::forget(pool);        
    res
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

