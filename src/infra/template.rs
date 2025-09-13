use std::sync::OnceLock;

use axum::http::{HeaderMap, StatusCode, header::CONTENT_TYPE};
use tera::{Context, Tera};
use tracing::error;

use crate::Result;

pub fn template() -> &'static Tera {
    static TEMPLATE: OnceLock<Tera> = OnceLock::new();

    TEMPLATE.get_or_init(|| match Tera::new("templates/**/*") {
        Ok(mut tera) => {
            tera.autoescape_on(vec![".html"]);
            tera
        }
        Err(err) => {
            error!("Error starting templating engine: {}", err);
            std::process::exit(1);
        }
    })
}

#[derive(Debug, Default)]
pub struct TemplateBuilder {
    status_code: Option<StatusCode>,
    template: Option<String>,
    headers: Option<HeaderMap>,
    context: Option<Vec<(String, String)>>,
    body: Option<String>,
}

impl TemplateBuilder {
    pub fn with_status_code(self, status_code: StatusCode) -> Self {
        Self {
            status_code: Some(status_code),
            ..self
        }
    }

    pub fn with_template(self, template: String) -> Self {
        Self {
            template: Some(template),
            ..self
        }
    }

    pub fn with_headers(self, with_headers: HeaderMap) -> Self {
        let mut headers = self.headers.unwrap_or_else(Self::default_headers);
        for (option_key, value) in with_headers {
            if let Some(key) = option_key {
                headers.insert(key, value);
            }
        }

        Self {
            headers: Some(headers),
            ..self
        }
    }

    pub fn with_context(self, context: Vec<(String, String)>) -> Self {
        Self {
            context: Some(context),
            ..self
        }
    }

    pub fn with_body(self, body: String) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }

    fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/html; charset=UTF-8".parse().unwrap());
        headers
    }

    pub fn build(self) -> Result<Template> {
        let status_code = self.status_code.unwrap_or(StatusCode::OK);
        let header_map = self.headers.unwrap_or_else(Self::default_headers);

        let context = match self.context {
            Some(vector) => {
                let mut context = Context::new();
                for (key, val) in vector {
                    context.insert(key, &val);
                }
                context
            }
            None => Context::new(),
        };

        let body = match self.template {
            Some(path) => template().render(&path, &context)?,
            None => self.body.unwrap_or_default(),
        };

        Ok((status_code, header_map, body))
    }
}

pub type Template = (StatusCode, HeaderMap, String);
