#[allow(unused_imports)]
use crate::{handler, model};

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "TODO",
        contact(name = "todo", url = "todo", email = "todo"),
        description = r#"説明"#,
    ),
    paths(handler::health::health_check, handler::health::health_check_db,),
    components(schemas())
)]
pub struct ApiDoc;
