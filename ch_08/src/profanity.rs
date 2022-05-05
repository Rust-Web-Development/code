use serde::{Deserialize, Serialize};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse(String);

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

pub async fn check_profanity(content: String) -> Result<String, handle_errors::Error> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        // Trace HTTP requests. See the tracing crate to make use of these traces.
        // Retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character={*}'")
        .header("apikey", "prmpFcctJu7duXweQA5zL5i0fIXmtzqF")
        .body(content)
        .send()
        .await
        .map_err(|e| handle_errors::Error::MiddlewareReqwestAPIError(e))?;
    
    if !res.status().is_success() {
        let status = res.status().as_u16();
        let message = res.json::<APIResponse>().await.unwrap();
    
        let err = handle_errors::APILayerError {
            status,
            message: message.0,
        };
    
        if status < 500 {
            return Err(handle_errors::Error::ClientError(err));
        } else {
            return Err(handle_errors::Error::ServerError(err));
        } 
    }
    
    match res.json::<BadWordsResponse>()
        .await {
            Ok(res) => Ok(res.censored_content),
            Err(e) => Err(handle_errors::Error::ReqwestAPIError(e)),
        }  
} 



