use graphql_client::{GraphQLQuery, Response};
use reqwest;
use serde::Serialize;
use thiserror::Error;
use cached::proc_macro::once;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "schema.json", query_path = "emotes.graphql")]
struct Emotes;

#[derive(Debug, Serialize, Clone)]
pub struct FinalEmote {
    name: String,
    url: String,
}

#[derive(Debug, Error)]
pub enum EmoteError {
    #[error("Response body extraction error")]
    ResponseError,

    #[error("Reqwest error when decoding response {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Unknown error")]
    Unknown,
}

const GRAPHQL_HOST: &str = "https://7tv.io/v3/gql";

fn get_emote_url(emote_data: emotes::EmotesEmoteSetEmotesDataHost) -> String {
    let host = emote_data.url;
    let files = emote_data
        .files
        .into_iter()
        .filter(|a| a.name.starts_with("4x"))
        .map(|a| a.name.to_string())
        .collect::<Vec<String>>();

    for file in &files {
        if file.contains(".gif") {
            return format!("{}/{}", host, file);
        }
    }

    return format!("https://{}/{}", host, files[0]);
}

async fn get_emotes_for_emote_set(
    variables: emotes::Variables,
) -> Result<Vec<FinalEmote>, EmoteError> {
    let request_body = Emotes::build_query(variables);

    let client = reqwest::Client::new();
    let res = client.post(GRAPHQL_HOST).json(&request_body).send().await?;
    let response_body: Response<emotes::ResponseData> = res.json().await?;

    let data = response_body.data.ok_or(EmoteError::ResponseError)?;
    Ok(data
        .emote_set
        .emotes
        .into_iter()
        .map(|item| FinalEmote {
            name: item.name,
            url: get_emote_url(item.data.host),
        })
        .collect())
}

#[once(time=10800, result=true)]
pub async fn get_emote_for_emote_set_id(
    emote_set_id: String,
) -> Result<Vec<FinalEmote>, EmoteError> {
    let variable = emotes::Variables {
        id: emote_set_id,
    };

    get_emotes_for_emote_set(variable).await
}

#[cfg(test)]
mod tests {
    use crate::seventv::get_emote_for_emote_set_id;

    #[tokio::test]
    async fn test_7tv() {
        println!("{:#?}", get_emote_for_emote_set_id("01J452JCVG0000352W25T9VEND".to_string()).await);
    }
}
