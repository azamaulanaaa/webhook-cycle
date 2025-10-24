pub mod api;
mod health;

use actix_web::web;
use utoipa::openapi::{OpenApiBuilder, Server};
use utoipa_actix_web::scope;
use utoipa_swagger_ui::{SwaggerUi, Url};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health)
        .service(scope("/api").service(scope("/v1").configure(api::v1::config)))
        .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (
                Url::new("v1", "/api-docs/v1/openapi.json"),
                OpenApiBuilder::from(api::v1::openapi())
                    .servers(Some([Server::new("/api/v1")]))
                    .build(),
                ),
        ]));
}
