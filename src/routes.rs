use crate::state::State;
use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use futures::stream::StreamExt;
use langchain_rust::chain::Chain;
use langchain_rust::prompt_args;
use serde::Deserialize;
use tera::{Context, Tera};

#[derive(Deserialize, Debug, Clone)]
struct PromptRequest {
    pub theme: String,
}

// TODO: simplify this
fn prepare_prompt(theme: &str) -> String {
    match theme {
        "ecology" => {
            "Propose moi une nouvelle idée de projet sur le thème de l'écologie.".to_string()
        }
        "3dprint" => {
            "Propose moi une nouvelle idée de projet sur le thème de l'impression 3D.".to_string()
        }
        "music" => {
            "Propose moi une nouvelle idée de projet sur le thème de la musique.".to_string()
        }
        "cooking" => {
            "Propose moi une nouvelle idée de projet sur le thème de la cuisine.".to_string()
        }
        _ => "Propose moi une nouvelle idée de projet.".to_string(),
    }
}

#[get("/")]
pub async fn home(data: web::Data<State>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("themes", &data.themes);

    let template = tera
        .render("index.html", &context)
        .expect("Fail to render package view");
    HttpResponse::Ok().body(template)
}

#[post("/prompt")]
async fn send_prompt(data: web::Data<State>, request: web::Json<PromptRequest>) -> impl Responder {
    let prompt = prepare_prompt(&request.theme);
    let input_variables = prompt_args! {
        "input" => prompt,
    };

    let stream_result = data.chain.stream(input_variables).await;
    match stream_result {
        Ok(stream) => {
            let stream = Box::pin(stream);
            let transformed_stream = stream.map(|result| match result {
                Ok(data) => Ok(actix_web::web::Bytes::from(data.content)),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
                    "Stream error: {:?}",
                    e
                ))),
            });

            HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .streaming(transformed_stream)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error creating stream: {:?}", e))
        }
    }
}
