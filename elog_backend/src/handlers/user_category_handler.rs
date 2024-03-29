use actix_web::{
    HttpResponse,
    web,
    get,
    post,
    put,
    delete
};
use crate::models::user_category::{
    UserCategory,
    NewUserCategory
};

use crate::utils::error_mapper::ElogError;
use crate::authentication::AuthenticatedRequest;

#[post("/user_category")]
pub async fn insert_user_category(
    authenticated_request: AuthenticatedRequest,
    mut new_user_category: web::Json<NewUserCategory>
) -> Result<HttpResponse, ElogError> {
    new_user_category.user_id = authenticated_request.user_id;
    UserCategory::insert(
        &authenticated_request.connection,
        new_user_category.0
    ).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/user_category")]
pub async fn get_all_user_categories(
    authenticated_request: AuthenticatedRequest
) -> Result<HttpResponse, ElogError> {
    UserCategory::get_list(
        &authenticated_request.connection,
        authenticated_request.user_id
    ).map(|list| {
        HttpResponse::Ok().json(list)
    })
}

#[put("/user_category/{user_category_id}")]
pub async fn update_user_category(
    authenticated_request: AuthenticatedRequest,
    dynamic_path: web::Path<(i16,)>,
    new_user_category: web::Json<NewUserCategory>
) -> Result<HttpResponse, ElogError> {
    UserCategory::update(
        &authenticated_request.connection,
        dynamic_path.into_inner().0,
        new_user_category.into_inner()
    ).map(|_| { HttpResponse::Ok().finish() })
}

#[delete("/user_category/{user_category_id}")]
pub async fn delete_user_category(
    authenticated_request: AuthenticatedRequest,
    dynamic_path: web::Path<(i16,)>,
) -> Result<HttpResponse, ElogError> {
    UserCategory::delete_by_id(&authenticated_request.connection, dynamic_path.into_inner().0).map(|_| {
        HttpResponse::Ok().finish()
    })
}
