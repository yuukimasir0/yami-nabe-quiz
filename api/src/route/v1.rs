use super::health::build_health_check_routers;
use axum::Router;
use registry::AppRegistry;

pub fn routes() -> Router<AppRegistry> {
    let router = Router::new().merge(build_health_check_routers());
    Router::new().nest("/api/v1", router)
}
