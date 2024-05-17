use crate::{state::State, theme::Theme};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use futures::stream::StreamExt;
use langchain_rust::chain::Chain;
use langchain_rust::prompt_args;
use serde::Deserialize;
use tera::{Context, Tera};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct PromptRequest {
    pub theme: String,
}

fn prepare_prompt(theme: &str, themes: &Vec<Theme>) -> String {
    let theme = themes.into_iter().find(|t| t.reference == theme);
    if let Some(theme) = theme {
        format!(
            "Propose moi une nouvelle idée de projet sur le thème {}.",
            theme.title
        )
    } else {
        "Propose moi une nouvelle idée de projet".to_owned()
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

#[utoipa::path(
  request_body = PromptRequest,
  responses(
      (status = 200, description = "AI response"),
      (status = 500, description = "Internal error")
  )
)]
#[post("/api/prompt")]
async fn send_prompt(data: web::Data<State>, request: web::Json<PromptRequest>) -> impl Responder {
    let prompt = prepare_prompt(&request.theme, &data.themes);
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
