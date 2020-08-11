use super::*;
use actix_web::{test, App};

#[actix_rt::test]
async fn test_status() {	
	let mut app = test::init_service(App::new().service(get_status)).await;
	let req = test::TestRequest::get().uri("/status").to_request();
	let resp = test::call_service(&mut app, req).await;
	assert!(resp.status().is_success());
	
	let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    assert_eq!(response_body, r##"{"result":true,"code":0,"description":"Everything is OK!"}"##);
}

#[actix_rt::test]
async fn test_get() {	
	let mut app = test::init_service(App::new().service(get_data_v1)).await;
	let req = test::TestRequest::get().uri("/data/v1/test").to_request();
	let resp = test::call_service(&mut app, req).await;
	assert!(resp.status().is_success());
	
	let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    assert_eq!(response_body, r##"{"result":true,"code":0,"description":"You requested get method with name: test"}"##);
}

#[actix_rt::test]
async fn test_post() {	
	let data = InputData{data: "test data".to_string()};
	let mut app = test::init_service(App::new().service(post_data_v1)).await;
	let req = test::TestRequest::post().header("content-type", "application/json").uri("/data/v1/test").set_json(&data).to_request();
	let resp = test::call_service(&mut app, req).await;
	assert!(resp.status().is_success());
	
	let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    assert_eq!(response_body, r##"{"result":true,"code":0,"description":"You requested post method with name: test, data is test data"}"##);
}

#[actix_rt::test]
async fn test_put() {	
	let data = InputData{data: "test data".to_string()};
	let mut app = test::init_service(App::new().service(put_data_v1)).await;
	let req = test::TestRequest::put().header("content-type", "application/json").uri("/data/v1/test").set_json(&data).to_request();
	let resp = test::call_service(&mut app, req).await;
	assert!(resp.status().is_success());
	
	let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    assert_eq!(response_body, r##"{"result":true,"code":0,"description":"You requested put method with name: test, data is test data"}"##);
}

#[actix_rt::test]
async fn test_delete() {	
	let data = InputData{data: "test data".to_string()};
	let mut app = test::init_service(App::new().service(delete_data_v1)).await;
	let req = test::TestRequest::delete().header("content-type", "application/json").uri("/data/v1/test").set_json(&data).to_request();
	let resp = test::call_service(&mut app, req).await;
	assert!(resp.status().is_success());
	
	let response_body = match resp.response().body().as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        _ => panic!("Response error"),
    };

    assert_eq!(response_body, r##"{"result":true,"code":0,"description":"You requested delete method with name: test, data is test data"}"##);
}