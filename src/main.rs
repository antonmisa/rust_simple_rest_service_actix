#[macro_use] extern crate serde_derive;
use actix_web::{web, get, post, put, delete, App, HttpServer, HttpResponse, Responder};

#[derive(Serialize, Deserialize)]
struct OutputData {
	result: bool,
	code:   u32,
	description: String,
}

#[derive(Serialize, Deserialize)]
struct InputData {
	data: String,
}

#[get("/status")]
async fn get_status() -> impl Responder {
    web::Json( OutputData {result: true, code: 0, description: "Everything is OK!".to_string()} )
}

#[get("/data/v1/{name}")]
async fn get_data_v1(name: web::Path<String>) -> impl Responder {
    web::Json( OutputData {result: true, code: 0, description: format!("You requested get method with name: {}", name)} )
}

#[post("/data/v1/{name}")]
async fn post_data_v1(data: web::Json<InputData>, name: web::Path<String>) -> impl Responder {
    web::Json( OutputData {result: true, code: 0, description: format!("You requested post method with name: {}, data is {}", name, data.data)} )
}

#[put("/data/v1/{name}")]
async fn put_data_v1(data: web::Json<InputData>, name: web::Path<String>) -> impl Responder {
    web::Json( OutputData {result: true, code: 0, description: format!("You requested put method with name: {}, data is {}", name, data.data)} )
}

#[delete("/data/v1/{name}")]
async fn delete_data_v1(data: web::Json<InputData>, name: web::Path<String>) -> impl Responder {
    web::Json( OutputData {result: true, code: 0, description: format!("You requested delete method with name: {}, data is {}", name, data.data)} )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
			App::new()
				.service(get_status)
				.service(get_data_v1)
				.service(post_data_v1)
				.service(put_data_v1)
				.service(delete_data_v1)
				.default_service(
                    web::route().to(|| HttpResponse::NotFound())
                )
		})
        .bind("0.0.0.0:8000")?
        .run()
		.await
}

#[cfg(test)]
mod test;