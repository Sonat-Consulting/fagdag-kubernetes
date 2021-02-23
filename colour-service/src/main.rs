#[macro_use]
extern crate log;

use crate::remote::get_remote;
use actix_web::middleware::Logger;
use actix_web::{http::ContentEncoding, web, Error, HttpResponse};
use env_logger::Env;
use handlebars::Handlebars;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::fs::File;
use std::fs;

mod remote;

async fn health_handler(state: web::Data<AppState>) -> HttpResponse {
    if !state.check_health() {
        return HttpResponse::InternalServerError().finish()
    }
    HttpResponse::Ok().finish()
}

async fn unhealthy_handler(state: web::Data<AppState>) -> HttpResponse {
    state.set_unhealthy();

    HttpResponse::Ok().body("Service is now unhealthy")
}

async fn stop_handler() -> HttpResponse {
    warn!("I have been asked to stop!");
    std::process::exit(99);
}

async fn index_handler(
    state: web::Data<AppState>,
    po: web::Data<PodInfo>,
    app_config: web::Data<AppConfig>,
    hb: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    state.counter.set(state.counter.get() + 1);

    let mut template_data = BTreeMap::new();
    let remote_service = app_config.get_ref();

    let pod_me = po.get_ref().clone();
    pod_me.number_of_requests.set(state.counter.get());
    template_data.insert("me", pod_me);
    if let Ok(pod_friend) = get_remote(&remote_service).await {
        info!(
            "Found friend pod! name={}, ip={}, colour={}",
            pod_friend.name, pod_friend.ip, pod_friend.colour_rgb
        );
        template_data.insert("friend", pod_friend);
    } else {
        warn!(
            "Unable to contact friend pod at {}",
            &app_config.remote_service
        )
    }

    let body = hb.render("index", &template_data).unwrap();
    info!("Generating template data={:?}", template_data);
    info!("Generated template:\n{}", body);

    HttpResponse::Ok().body(body)
}

async fn info_handler(
    state: web::Data<AppState>,
    pod_info: web::Data<PodInfo>,
    request: web::HttpRequest,
) -> Result<HttpResponse, Error> {
    info!("Sending pod info to {:?}", &request.connection_info());
    let pod_me = pod_info.get_ref();
    pod_me.number_of_requests.set(state.counter.get());
    Ok(HttpResponse::Ok().json(pod_info.get_ref()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware, web, App, HttpServer};
    let app_config = AppConfig::new();
    app_config.new_logger();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    handlebars.set_strict_mode(true);
    let handlebars_ref = web::Data::new(handlebars);

    let pod_info = PodInfo::new();
    info!(
        "Starting with app_config={:?} pod_info={:?}",
        &app_config, &pod_info
    );

    HttpServer::new(move || {
        App::new()
            .wrap(app_config.new_middleware_logger())
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .data(AppState::new())
            .app_data(handlebars_ref.clone())
            .data(pod_info.clone())
            .data(app_config.clone())
            .route("/info", web::get().to(info_handler))
            .route("/crash", web::get().to(stop_handler))
            .route("/health", web::get().to(health_handler))
            .route("/unhealthy", web::get().to(unhealthy_handler))
            // Default / and wildcard handler
            .route("/{tail:.*}", web::get().to(index_handler))
    })
    .workers(1)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    log_level: String,
    log_health: bool,
    pub remote_service: String,
}

impl AppConfig {
    fn new() -> AppConfig {
        return AppConfig {
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            log_health: match std::env::var("LOG_HEALTH_ENDPOINT") {
                Ok(s) => {
                    let mut log_h = false;
                    if let Ok(r) = bool::from_str(s.as_str()) {
                        log_h = r;
                    }
                    log_h
                }
                _ => false,
            },
            remote_service: format!(
                "http://{}:8080/info",
                std::env::var("REMOTE_SERVICE").unwrap_or_else(|_| "localhost".to_string())
            ),
        };
    }

    fn new_logger(&self) {
        env_logger::Builder::from_env(Env::default().default_filter_or(&self.log_level)).init();
    }

    fn new_middleware_logger(&self) -> Logger {
        let mut logger = Logger::default();
        if !self.log_health {
            logger = logger.exclude_regex("/health");
        }
        logger
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PodInfo {
    node: String,
    name: String,
    name_space: String,
    ip: String,
    number_of_requests: Cell<usize>,
    colour_rgb: String,
    new_colour: Option<String>
}

impl PodInfo {
    fn new() -> PodInfo {
        let mut rng = rand::thread_rng();

        let colour_rgb = format!(
            "rgb({},{},{})",
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255)
        );

        PodInfo {
            node: std::env::var("NODE_NAME").unwrap_or_else(|_| "NODE_NAME NOT FOUND".to_string()),
            name: std::env::var("POD_NAME").unwrap_or_else(|_| "POD_NAME NOT FOUND".to_string()),
            name_space: std::env::var("POD_NAMESPACE")
                .unwrap_or_else(|_| "POD_NAMESPACE NOT FOUND".to_string()),
            ip: std::env::var("POD_IP").unwrap_or_else(|_| "POD_IP NOT FOUND".to_string()),
            number_of_requests: Cell::new(0),
            colour_rgb,
            new_colour: None
        }
    }
}

struct AppState {
    counter: Cell<usize>,
}

static STATE_NAME: &str = "/app/healthy";
impl AppState {
    fn new() -> AppState {
        AppState {
            counter: Cell::new(0),
        }
    }

    fn check_health(&self) -> bool {
        if let Ok(_) = File::open(STATE_NAME) {
            return true;
        }
        false
    }

    fn set_unhealthy(&self) {
        if let Err(e) = fs::remove_file(STATE_NAME) {
            warn!("could not create unhealthy file! {:?}", e)
        }
        info!("service is sat to unhealthy now!")
    }
}
