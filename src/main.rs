use actix_files::Files;
use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use side_project_generator::{routes, state::load_state};
use std::env;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_template("index.html", include_str!("./public/index.html"))
            .expect("Expected template");
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ollama_base_url =
        env::var("OLLAMA_BASE_URL").unwrap_or("http://localhost:11434".to_string());

    let model = env::var("LLM_MODEL").unwrap_or("llama3".to_string());

    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("Invalid port");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .app_data(load_state(&ollama_base_url, &model).unwrap())
            .service(routes::home)
            .service(routes::send_prompt)
            .service(
                Files::new("/assets", "src/public/assets")
                    .prefer_utf8(true)
                    .use_last_modified(true),
            )
    })
    .bind(("127.0.0.1", port))?
    .run();

    println!("Application running on http://localhost:{}", port);

    server.await
}
