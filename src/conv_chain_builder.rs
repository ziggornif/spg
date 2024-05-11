use langchain_rust::{
    chain::{builder::ConversationalChainBuilder, ConversationalChain},
    fmt_message, fmt_template,
    llm::{openai::OpenAI, OpenAIConfig},
    memory::SimpleMemory,
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    schemas::Message,
    template_fstring,
};

/**
 * Build conversation chain
 *
 * This function configure :
 * - ollama connection
 * - prompt sent to the model
 * - conversation memory
 * - conversational chain instance
 */
pub fn new_conv_chain(ollama_base_url: &str, model: &str) -> ConversationalChain {
    let llm = OpenAI::default()
        .with_config(
            OpenAIConfig::default()
                .with_api_base(format!("{}/v1", ollama_base_url))
                .with_api_key("ollama"),
        )
        .with_model(model);

    let memory = SimpleMemory::new();

    let prompt = message_formatter![
        fmt_message!(Message::new_system_message(
            "Tu dois impérativement répondre aux questions en français, c'est primordial. Tu es une IA spécialisée dans la création de side projects (projets perso) informatiques, ton objectif est d'aider les développeurs à trouver des idées de projets à créer. Lorsque tu proposeras des idées de projets à créer, tu devras détailler le produit et son fonctionnement."
        )),
        fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
            "{input}", "input"
        ))),
    ];

    ConversationalChainBuilder::new()
        .llm(llm)
        .prompt(prompt)
        .memory(memory.into())
        .build()
        .expect("Error building ConversationalChain")
}
