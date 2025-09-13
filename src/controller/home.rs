use tracing::instrument;

use crate::{
    Result,
    infra::template::{Template, TemplateBuilder},
};

#[instrument]
pub async fn index() -> Result<Template> {
    TemplateBuilder::default()
        .with_template(String::from("home/index.html"))
        .with_context(vec![(
            String::from("context"),
            String::from("Hello World!"),
        )])
        .build()
}
