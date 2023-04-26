use poem_openapi::{
    payload::{Binary, Json, PlainText, Response},
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
    #[oai(
        status = 200,
        header(
            name = "Content-Disposition",
            type = "String",
            description = "Content-Disposition header"
        )
    )]
    Ok(Binary<Vec<u8>>),
    #[oai(status = 500)]
    InternalError(PlainText<String>),
}

impl BinaryResponse {
    pub fn ok(payload: Binary<Vec<u8>>, filename: String, extension: String) -> Response<Self> {
        Response::new(Self::Ok(payload)).header(
            "Content-Disposition",
            format!("file=\"{filename}.{extension}\""),
        )
    }
    pub fn internal_error(error: String) -> Self {
        Self::InternalError(PlainText(error))
    }
}

impl From<BinaryResponse> for Response<BinaryResponse> {
    fn from(response: BinaryResponse) -> Self {
        Self::new(response)
    }
}
