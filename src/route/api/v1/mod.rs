use utoipa::openapi::{Components, Info, OpenApi, OpenApiBuilder};
use utoipa_actix_web::{OpenApiFactory, scope, service_config::ServiceConfig};

pub fn config(_cfg: &mut ServiceConfig) {}

pub fn openapi() -> OpenApi {
    let scope = scope("").configure(config);

    let components = {
        let mut schemas = Vec::new();
        scope.schemas(&mut schemas);

        Components::builder().schemas_from_iter(schemas).build()
    };

    OpenApiBuilder::new()
        .info(Info::new("webhook-cycle", "1.0.0"))
        .paths(scope.paths())
        .components(Some(components))
        .build()
}
