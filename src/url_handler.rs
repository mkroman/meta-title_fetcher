use std::sync::Mutex;

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

lazy_static! {
    // Url handler registry
    static ref URL_HANDLER_REGISTRY: Mutex<UrlHandlerRegistry> = Mutex::new(UrlHandlerRegistry::new());
}

pub fn init() {
    let mut url_handler_registry = URL_HANDLER_REGISTRY.lock().unwrap();

    url_handler_registry.register(html_title::HtmlTitleUrlHandler::new());
}
