use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;
use crate::AppState;
use super::ApiResponse;

#[derive(Deserialize)]
pub struct IssueCertificateRequest {
    pub course_id: Uuid,
    pub student_address: String,
}

pub async fn issue_certificate(
    state: web::Data<AppState>,
    req: web::Json<IssueCertificateRequest>,
) -> impl Responder {
    match state.course_service.issue_certificate(
        req.course_id,
        req.student_address.clone(),
    ).await {
        Ok(cert) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(cert),
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
        web::scope("/certificates")
            .route("", web::post().to(issue_certificate))
    );
}
