use poem_openapi::{
    payload::{Binary, Json, PlainText},
    types::{ParseFromJSON, ToJSON},
    ApiResponse,
};

#[derive(ApiResponse)]
pub enum JsonResponse<T: ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<T>),
    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> JsonResponse<T> {
    pub fn ok(payload: T) -> Self {
        Self::Ok(Json(payload))
    }
    pub fn internal_error(error: String) -> Self {
        Self::InternalError(PlainText(error))
    }
}

#[derive(ApiResponse)]
pub enum BinaryResponse {
    #[oai(status = 200)]
    Ok(Binary<Vec<u8>>),
    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

impl BinaryResponse {
    pub fn ok(payload: Binary<Vec<u8>>) -> Self {
        Self::Ok(payload)
    }
    pub fn internal_error(error: String) -> Self {
        Self::InternalError(PlainText(error))
    }
}
