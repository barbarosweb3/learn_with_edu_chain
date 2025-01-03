use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::AppState;
use super::ApiResponse;

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub metadata: String,
    pub price: i64,
}

pub async fn create_course(
    state: web::Data<AppState>,
    req: web::Json<CreateCourseRequest>,
) -> impl Responder {
    match state.course_service.create_course(
        req.title.clone(),
        req.metadata.clone(),
        req.price,
        "instructor_address".to_string(), // In real app, get from auth
    ).await {
        Ok(course) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(course),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("", web::post().to(create_course))
    );
}
