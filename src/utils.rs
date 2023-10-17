use cfg_if::cfg_if;
use worker::{wasm_bindgen::JsValue, *};

cfg_if! {
    // Check if the "console_error_panic_hook" feature is enabled
    if #[cfg(feature = "console_error_panic_hook")] {
        // If enabled, import the console_error_panic_hook crate
        extern crate console_error_panic_hook;
        // Export the set_once function from the crate as set_panic_hook
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        // If the feature is not enabled, define a no-op function for set_panic_hook
        #[inline]
        pub fn set_panic_hook() {}
    }
}

// Define an asynchronous function named mongo_request that takes input parameters
pub async fn mongo_request(
    method: worker::Method,
    ctx: &RouteContext<()>,
    body: &str,
    query_type: &str,
    query: &str,
) -> Result<String> {
    // Retrieve the "mongo_data_api_key" secret from the context and convert it to a string
    let api_key = ctx.secret("mongo_data_api_key")?.to_string();
    
    // Build the URL for the MongoDB API request
    let url = format!(
        "https://ap-south-1.aws.data.mongodb-api.com/app/data-lqfly/endpoint/data/v1/{}/{}",
        query_type, query
    );

    // Create a new set of headers and set the required headers
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Request-Headers", "*")?;
    headers.set("Content-Type", "application/json")?;
    headers.set("api-key", api_key.as_str())?;

    // Create a new request initialization with the specified method, headers, and body
    let mut request = RequestInit::new();
    request
        .with_method(method)
        .with_headers(headers)
        .with_body(Some(JsValue::from_str(body)));

    let mut response = Fetch::Request(Request::new_with_init(&url, &request)?)
        .send()
        .await?;

        response.text().await
}
