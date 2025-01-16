use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

use swarm_agent::{Agent, AgentConfig};
use swarm_wallet::{Wallet, WalletError, evm::EvmWallet};
use swarm_tool::{Tool, ParameterType, ToolSchema, ToolParameter};

use crate::tools::yahoo_finance::YahooFinanceTool;


pub struct BtcPriceStatisticalPredictorAgent {
    config: AgentConfig,
    tools: HashMap<String, Box<dyn Tool>>,
    wallet: Option<Box<dyn Wallet>>,
    schema: ToolSchema,
}

impl std::fmt::Debug for BtcPriceStatisticalPredictorAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BtcPriceStatisticalPredictorAgent")
            .field("config", &self.config)
            .field("tools", &self.tools.keys().collect::<Vec<_>>())
            .field("schema", &self.schema)
            .finish()
    }
}

impl BtcPriceStatisticalPredictorAgent {
    pub fn new(config: AgentConfig) -> Self {
        let mut tools = HashMap::new();
        tools.insert(
            "yahoo-finance".to_string(),
            Box::new(YahooFinanceTool::new()) as Box<dyn Tool>,
        );

        Self {
            config,
            tools,
            wallet: None,
            schema: ToolSchema {
                name: "btc-price-statistical-predictor".to_string(),
                description: "Predicts BTC price using statistical analysis".to_string(),
                parameters: vec![ToolParameter {
                    name: "days".to_string(),
                    description: "Number of days for prediction".to_string(),
                    parameter_type: ParameterType::Integer,
                    required: true,
                }],
            },
        }
    }

    async fn calculate_moving_average(&self, data: &str) -> Result<f64, String> {
        #[derive(serde::Deserialize)]
        struct HistoricalData {
            close: f64,
        }

        let historical_data: Vec<HistoricalData> = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse historical data: {}", e))?;

        if historical_data.is_empty() {
            return Err("No historical data available".to_string());
        }

        let sum: f64 = historical_data.iter().map(|entry| entry.close).sum();
        Ok(sum / historical_data.len() as f64)
    }
}

#[async_trait]
impl Agent for BtcPriceStatisticalPredictorAgent {
    async fn run(&mut self, task: &str) -> Result<String, String> {
        // Parse number of days from task
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
        let prediction = self.calculate_moving_average(&historical_data)
            .await
            .map(|val| format!("{}", val))?;

        // Format prediction
        let result = format!("BTC Price Prediction (Moving Average): ${:.2}", prediction);

        Ok(result)
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


#[cfg(test)]
mod tests {
    use super::*;

    // Statistical Predictor Tests
    #[tokio::test]
    async fn test_statistical_predictor_basic() {
        let config = AgentConfig::default();
        let agent = BtcPriceStatisticalPredictorAgent::new(config);

        assert!(agent.name().contains("agent"));
    }

    #[tokio::test]
    async fn test_statistical_predictor_validation() {
        let config = AgentConfig::default();
        let mut agent = BtcPriceStatisticalPredictorAgent::new(config);

        // Test invalid input (no days)
        let result = agent.run("predict BTC price").await;
        assert!(result.is_err());

        // Test invalid days format
        let result = agent.run("predict BTC price in invalid days").await;
        println!("statistical predictor validation: {:?}", result);
        assert!(result.is_err());
    }
}