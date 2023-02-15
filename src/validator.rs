use std::sync::Arc;

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequestParts, RawBody},
    http::request::Parts,
    Json as JsonAxum, TypedHeader, body::{Body, HttpBody},
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

use crate::AppState;

pub struct ValidJson<T>(pub T)
where
    T: Validate;

#[async_trait]
impl<T> FromRequestParts<Arc<AppState>> for ValidJson<T>
where
    T: Validate + Sized + DeserializeOwned,
{
    type Rejection = JsonValidatorRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        match RawBody::from_request_parts(parts, state).await {
            Ok(json_value) => {
                if let Err(e) = json_value.validate() {
                    return Err(JsonValidatorRejection::ValidationRejection(e));
                }
                return Ok(ValidJson(json_value.0));
            }
            Err(e) => return Err(JsonValidatorRejection::JsonAxumRejection(e)),
        };
    }
}

/// Errors returned on validation.
pub enum JsonValidatorRejection {
    /// Axum's own validation errors.
    JsonAxumRejection(JsonRejection),
    /// The one returned from the validator.
    ValidationRejection(ValidationErrors),
}
