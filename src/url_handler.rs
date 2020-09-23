mod html_title;

pub trait UrlHandler {}

pub struct UrlHandlerRegistry {
    pub url_handlers: Vec<Box<dyn UrlHandler + Sync + Send>>,
}

impl UrlHandlerRegistry {
    pub fn new() -> UrlHandlerRegistry {
        UrlHandlerRegistry {
            url_handlers: vec![],
        }
    }

    pub fn register(&mut self, url_handler: impl UrlHandler + Sync + Send + 'static) {
        self.url_handlers.push(Box::new(url_handler));
    }
}

pub fn init_registry() -> UrlHandlerRegistry {
    let mut registry = UrlHandlerRegistry::new();

    html_title::register(&mut registry);

    registry
}
