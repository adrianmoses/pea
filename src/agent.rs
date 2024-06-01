use async_openai::{
    Client as OpenAIClient, config::OpenAIConfig
};

pub trait Agent {
    fn name(&self) -> String;
    fn client(&self) -> OpenAIClient<OpenAIConfig>;
    fn system_message(&self) -> String;

    async fn prompt(&self, input: &str, data: String) -> Result<String, anyhow::Error>;
}


#[derive(Clone)]
pub struct Planner {
    http_client: reqwest::Client,
    system: Option<String>,
    openai_client: OpenAIClient<OpenAIConfig>
}

#[derive(Clone)]
pub struct Scheduler {
    http_client: reqwest::Client,
    system: Option<String>,
    openai_client: OpenAIClient<OpenAIConfig>
}

#[derive(Clone)]
pub struct Executor {
    http_client: reqwest::Client,
    system: Option<String>,
    openai_client: OpenAIClient<OpenAIConfig>
}