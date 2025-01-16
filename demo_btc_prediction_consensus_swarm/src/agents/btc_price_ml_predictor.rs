use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use swarm_agent::{
    service::{ServiceAgent, ServiceAgentConfig, ServiceError},
    Agent, AgentConfig,
};
use swarm_tool::{ParameterType, Tool, ToolParameter, ToolSchema};
use swarm_wallet::{evm::EvmWallet, Wallet, WalletError};

use crate::tools::yahoo_finance::YahooFinanceTool;

#[derive(Debug)]
struct LinearRegression {
    weights: Vec<f64>,
    bias: f64,
}

impl LinearRegression {
    fn new() -> Self {
        LinearRegression {
            weights: Vec::new(),
            bias: 0.0,
        }
    }

    fn fit(
        &mut self,
        x: &[Vec<f64>],
        y: &[f64],
        learning_rate: f64,
        epochs: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let n_samples = x.len();
        let n_features = x[0].len();

        // Initialize weights and bias
        self.weights = vec![0.0; n_features];
        self.bias = 0.0;

        for _ in 0..epochs {
            let mut weight_gradients = vec![0.0; n_features];
            let mut bias_gradient = 0.0;

            for i in 0..n_samples {
                let prediction = self.predict_single(&x[i]);
                let error = prediction - y[i];

                // Calculate gradients
                for j in 0..n_features {
                    weight_gradients[j] += error * x[i][j];
                }
                bias_gradient += error;
            }

            // Update weights and bias
            for j in 0..n_features {
                self.weights[j] -= learning_rate * weight_gradients[j] / n_samples as f64;
            }
            self.bias -= learning_rate * bias_gradient / n_samples as f64;
        }

        Ok(())
    }

    fn predict(&self, x: &[Vec<f64>]) -> Vec<f64> {
        x.iter().map(|sample| self.predict_single(sample)).collect()
    }

    fn predict_single(&self, x: &[f64]) -> f64 {
        x.iter()
            .zip(&self.weights)
            .map(|(xi, wi)| xi * wi)
            .sum::<f64>()
            + self.bias
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HistoricalData {
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PredictionRequest {
    days: i64,
    historical_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PredictionResponse {
    price: f64,
    confidence: f64,
    model_version: String,
}

pub struct BtcPriceMLPredictorAgent {
    config: AgentConfig,
    service_config: ServiceAgentConfig,
    client: Client,
    tools: HashMap<String, Box<dyn Tool>>,
    wallet: Option<Box<dyn Wallet>>,
}

impl BtcPriceMLPredictorAgent {
    pub fn new(config: AgentConfig, service_config: ServiceAgentConfig) -> Self {
        let mut tools = HashMap::new();
        tools.insert(
            "yahoo-finance".to_string(),
            Box::new(YahooFinanceTool::new()) as Box<dyn Tool>,
        );

        Self {
            config,
            service_config,
            client: Client::new(),
            tools,
            wallet: None,
        }
    }

    pub fn with_wallet(mut self, wallet: Box<dyn Wallet>) -> Self {
        self.wallet = wallet.into();
        self
    }

    async fn get_prediction(
        &self,
        days: i64,
        historical_data: &str,
    ) -> Result<PredictionResponse, Box<dyn std::error::Error>> {
        // In test mode, return mock prediction
        if self.config.test_mode {
            return Ok(PredictionResponse {
                price: 50000.0,
                confidence: 0.85,
                model_version: "mock-ml-v1".to_string(),
            });
        }

        let request = PredictionRequest {
            days,
            historical_data: historical_data.to_string(),
        };

        let parsed_data: Vec<HistoricalData> = serde_json::from_str(&historical_data)?;

        let x: Vec<Vec<f64>> = parsed_data
            .iter()
            .map(|data| vec![data.open, data.high, data.low, data.volume as f64])
            .collect();
        let y: Vec<f64> = parsed_data.iter().map(|data| data.close).collect();

        // Create and train the model
        let mut model = LinearRegression::new();
        model.fit(&x, &y, 0.01, 1000)?;

        // Predict the BTC price 5 days into the future
        let future_features = x.last().ok_or("No data available for prediction")?;
        let prediction = model.predict_single(future_features);

        let response = self
            .client
            .post(&self.service_config.endpoint)
            .json(&request)
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.service_config
                        .api_key
                        .as_ref()
                        .unwrap_or(&"".to_string())
                ),
            )
            .send()
            .await
            .map_err(|e| format!("Failed to send prediction request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Service returned error: {}", response.status()).into());
        }

        Ok(response
            .json::<PredictionResponse>()
            .await
            .map_err(|e| format!("Failed to parse prediction response: {}", e))?)
    }
}

#[async_trait]
impl Agent for BtcPriceMLPredictorAgent {
    async fn run(&mut self, task: &str) -> Result<String, String> {
        let days = task
            .split_whitespace()
            .find(|s| s.parse::<i64>().is_ok())
            .and_then(|s| s.parse::<i64>().ok())
            .ok_or_else(|| "Task must include number of days".to_string())?;

        // Get historical data using YahooFinanceApiTool
        let yahoo_tool = self
            .tools
            .get("yahoo-finance")
            .ok_or_else(|| "Yahoo Finance tool not found".to_string())?;

        let historical_data: String = yahoo_tool.run(&days.to_string())?;
        let prediction = self
            .get_prediction(days, &historical_data)
            .await
            .map_err(|e| e.to_string())?;

        let result = format!(
            "BTC Price Prediction (ML Model v{}): ${:.2} (Confidence: {:.2})",
            prediction.model_version, prediction.price, prediction.confidence
        );

        if let Some(ref identity) = self.wallet {
            let signature = identity
                .sign_message(&result)
                .await
                .map_err(|e| e.to_string())?;
            let address = identity.address();
            if address.is_empty() {
                return Err("No wallet address available".to_string());
            }

            Ok(format!(
                "Prediction: {}\nSignature: {}\nAddress: {}",
                result, signature, address
            ))
        } else {
            Ok(result)
        }
    }

    fn id(&self) -> Uuid {
        self.config.id
    }

    fn name(&self) -> &str {
        &self.config.name
    }

    fn system_prompt(&self) -> &str {
        &self.config.system_prompt
    }

    fn add_tool(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    fn config(&self) -> &AgentConfig {
        &self.config
    }
}

#[async_trait]
impl ServiceAgent for BtcPriceMLPredictorAgent {
    fn service_config(&self) -> &ServiceAgentConfig {
        &self.service_config
    }

    async fn health_check(&self) -> Result<bool, ServiceError> {
        if let Some(health_endpoint) = &self.service_config.health_endpoint {
            match self.client.get(health_endpoint).send().await {
                Ok(response) => Ok(response.status().is_success()),
                Err(_) => Ok(false),
            }
        } else {
            Ok(true)
        }
    }

    async fn sign_message(&self, output: &str) -> Result<String, ServiceError> {
        if let Some(ref wallet) = self.wallet {
            let signature = wallet
                .sign_message(output)
                .await
                .map_err(|arg0: WalletError| {
                    ServiceError::IdentityVerificationFailed(arg0.to_string())
                })?;
            let address = wallet.address();
            if address.is_empty() {
                return Err(ServiceError::IdentityVerificationFailed(
                    "No wallet address".to_string(),
                ));
            }

            Ok(format!(
                "Output: {}\nSignature: {}\nAddress: {}",
                output, signature, address
            ))
        } else {
            Err(ServiceError::IdentityVerificationFailed(
                "No wallet configured".to_string(),
            ))
        }
    }

    async fn verify_message(
        &self,
        output: &str,
        signature: &str,
        agent_address: &str,
    ) -> Result<bool, ServiceError> {
        if let Some(ref wallet) = self.wallet {
            wallet
                .verify_signature(output, signature, agent_address)
                .await
                .map_err(|e| {
                    ServiceError::IdentityVerificationFailed(format!(
                        "Signature verification failed: {}",
                        e
                    ))
                })
        } else {
            Err(ServiceError::IdentityVerificationFailed(
                "No wallet configured".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    
    // ML Predictor Tests
    #[tokio::test]
    async fn test_ml_predictor_basic() {
        let config = AgentConfig {
            test_mode: true,
            ..AgentConfig::default()
        };
        let service_config = ServiceAgentConfig {
            endpoint: "http://localhost:8080".to_string(),
            timeout: Duration::from_secs(5),
            ..ServiceAgentConfig::default()
        };
        let mut agent = BtcPriceMLPredictorAgent::new(config, service_config);

        assert!(agent.name().contains("agent"));
        assert!(agent.service_config().endpoint == "http://localhost:8080");
    }

    #[tokio::test]
    async fn test_ml_predictor_validation() {
        let config = AgentConfig {
            test_mode: true,
            ..AgentConfig::default()
        };
        let service_config = ServiceAgentConfig::default();
        let mut agent = BtcPriceMLPredictorAgent::new(config, service_config);

        // Test invalid input (no days)
        let result = agent.run("predict BTC price").await;
        assert!(result.is_err());

        // Test invalid days format
        let result = agent.run("predict BTC price in invalid days").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ml_predictor_health_check() {
        let config = AgentConfig {
            test_mode: true,
            ..AgentConfig::default()
        };
        let service_config = ServiceAgentConfig {
            health_endpoint: Some("http://localhost:8080/health".to_string()),
            ..ServiceAgentConfig::default()
        };
        let agent = BtcPriceMLPredictorAgent::new(config, service_config);

        let result = agent.health_check().await;
        assert!(result.is_ok());
    }
}
