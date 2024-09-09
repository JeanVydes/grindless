use actix_web::{
    body::BoxBody, http::header::ContentType, web, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use super::errors::Errors;

/// # Success
///
/// This enum is used to determine if the request was successful or not.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Success {
    Ok,
    Error,
}

/// # ResponseObject
///
/// This struct is used to wrap the response object that is returned from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseObject<T> {
    pub success: Success,
    pub message: Option<String>,
    pub data: Option<T>,
    pub errors: Option<Vec<ResponseObjectError>>,
}

/// # ResponseObjectError
///
/// This struct is used to wrap the error/s that is returned from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseObjectError {
    pub error_id: Errors,
    pub message: Option<String>,
}

/// # Responder impl for Response from actix_web
///
/// This is the implementation of the Responder trait for the Response struct.
impl<T> Responder for ResponseObject<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub struct ResponseBuilderOk<T> {
    pub message: Option<String>,
    pub data: Option<T>,
}

pub struct ResponseBuilderError {
    pub message: String,
    pub errors: Vec<ResponseObjectError>,
}

pub fn build_ok<T>(response_builder_ok: ResponseBuilderOk<T>) -> web::Json<ResponseObject<T>> {
    let response_object = ResponseObject {
        success: Success::Ok,
        message: response_builder_ok.message,
        data: response_builder_ok.data,
        errors: None,
    };

    web::Json(response_object)
}

pub fn build_err<T>(response_builder_error: ResponseBuilderError) -> web::Json<ResponseObject<T>> {
    let response_object = ResponseObject {
        success: Success::Error,
        message: Some(response_builder_error.message),
        data: None,
        errors: Some(response_builder_error.errors),
    };

    web::Json(response_object)
}
