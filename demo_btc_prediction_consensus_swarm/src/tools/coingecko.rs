use async_trait::async_trait;
use reqwest::Client;
use swarm_tool::{Tool, ToolSchema};

#[derive(Debug)]
pub struct CoingeckoTool {
    client: Client,
    schema: ToolSchema,
    base_url: String,
}

impl Default for CoingeckoTool {
    fn default() -> Self {
        Self::new()
    }
}

impl CoingeckoTool {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            schema: ToolSchema {
                name: "coingecko-api".to_string(),
                description: "Retrieves current BTC price from Coingecko API".to_string(),
                parameters: vec![],
            },
            base_url: "https://api.coingecko.com/api/v3".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
}

impl CoingeckoTool {
    async fn run(&self, _input: &str) -> Result<String, String> {
        let url = format!(
            "{}/simple/price?ids=bitcoin&vs_currencies=usd",
            self.base_url
        );

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        //println!("Response: {:?}", response);

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let price = json["bitcoin"]["usd"]
            .as_f64()
            .ok_or_else(|| format!("Invalid response format: {}", json))?;

        Ok(price.to_string())
    }    
}

#[async_trait]
impl Tool for CoingeckoTool {
    fn name(&self) -> &str {
        &self.schema.name
    }

    fn description(&self) -> &str {
        &self.schema.description
    }

    fn schema(&self) -> &ToolSchema {
        &self.schema
    }

    fn run(&self, input: &str) -> Result<String, String> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.run(input))
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use wiremock::matchers::method;
    use wiremock::matchers::path;
    use wiremock::ResponseTemplate;
    use wiremock::MockServer;
    use wiremock::Mock;
    use super::*;

    #[tokio::test]
    async fn test_coingecko_api_tool() {
        let tool = CoingeckoTool::new();
        assert_eq!(tool.name(), "coingecko-api");
    }

    #[tokio::test]
    async fn test_coingecko_api_tool_run_mock() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Mock the Coingecko API response
        Mock::given(method("GET"))
            .and(path("/simple/price"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "bitcoin": {
                    "usd": 50000
                }
            })))
            .mount(&mock_server)
            .await;

        let tool = CoingeckoTool::new().with_base_url(mock_server.uri());
        let result = tool.run("").await;
        println!("CoingeckoApiTool mock run result: {:?}", result);

        assert!(result.is_ok());
        let price = result.unwrap().parse::<f64>().unwrap();
        assert!(price > 0.0);
    }

    #[tokio::test]
    async fn test_coingecko_api_tool_run() {
        let tool = CoingeckoTool::new();
        let result = tool.run("").await;
        println!("CoingeckoApiTool run result: {:?}", result);

        assert!(result.is_ok());
        let price = result.unwrap().parse::<f64>().unwrap();
        assert!(price > 0.0);
    }
}
