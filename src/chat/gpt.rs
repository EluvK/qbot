pub(crate) use req::GptReqParam;
pub(crate) use resp::GptRecvResult;

pub async fn send_to_gpt(body: String, url: String, token: &str) -> anyhow::Result<GptRecvResult> {
    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .body(body)
        .send()
        .await
        .map_err(|err| anyhow::anyhow!("client error: {err}"))?
        .text()
        .await
        .map_err(|err| anyhow::anyhow!("client recv test err: {err}"))?;
    Ok(serde_json::from_str::<GptRecvResult>(&resp)?)
}

pub(crate) mod req {
    pub(crate) struct GptReqParam {
        pub(crate) body: String,
        pub(crate) url: String,
    }
}

pub(crate) mod resp {
    use std::fmt::Display;

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub(crate) struct Usage {
        pub(crate) prompt_tokens: i64,

        pub(crate) completion_tokens: i64,
        pub(crate) total_tokens: i64,
    }

    impl Display for Usage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Prompt tokens used: {}, Completion tokens used: {}, Total tokens used: {}",
                self.prompt_tokens, self.completion_tokens, self.total_tokens
            )
        }
    }

    #[allow(unused)]
    #[derive(Debug, Deserialize)]
    pub(crate) struct Message {
        role: crate::chat::message::MessageRole,
        pub(crate) content: String,
    }

    #[derive(Debug, Deserialize)]
    pub(crate) struct Choices {
        pub(crate) message: Message,
    }

    #[allow(unused)]
    #[derive(Debug, Deserialize)]
    pub(crate) struct GptRecvResult {
        pub(crate) id: String,
        pub(crate) object: String,
        pub(crate) created: u64,
        pub(crate) model: String,
        pub(crate) choices: Vec<Choices>,
        pub(crate) usage: Usage,
    }
}
