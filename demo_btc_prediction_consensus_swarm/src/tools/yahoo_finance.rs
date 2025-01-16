use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use swarm_tool::{Tool, ToolSchema, ParameterType, ToolParameter};
use yahoo_finance_api::time::{self, OffsetDateTime};
use yahoo_finance_api::YahooConnector;

#[derive(Debug, Serialize, Deserialize)]
struct HistoricalData {
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, Result as SqliteResult};
use std::path::PathBuf;

#[derive(Debug)]
pub struct YahooFinanceTool {
    schema: ToolSchema,
    db_path: PathBuf,
}

impl Default for YahooFinanceTool {
    fn default() -> Self {
        Self::new()
    }
}

impl YahooFinanceTool {
    pub fn new() -> Self {
        Self {
            schema: ToolSchema {
                name: "yahoo-finance".to_string(),
                description: "Retrieves historical BTC price data from Yahoo Finance".to_string(),
                parameters: vec![ToolParameter {
                    name: "days".to_string(),
                    description: "Number of days of historical data".to_string(),
                    parameter_type: ParameterType::Integer,
                    required: true,
                }],
            },
            db_path: PathBuf::from("cache.db"),
        }
    }

    pub fn with_db_path(mut self, path: PathBuf) -> Self {
        self.db_path = path;
        self
    }

    async fn get_cached_data(&self, symbol: &str, _days: i64) -> Result<Option<String>, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Failed to open cache database: {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS price_cache (
                symbol TEXT NOT NULL,
                data TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("Failed to create cache table: {}", e))?;

        let cutoff = (Utc::now() - chrono::Duration::days(1)).timestamp();
        let mut stmt = conn
            .prepare(
                "SELECT data FROM price_cache 
                WHERE symbol = ? AND timestamp > ? 
                ORDER BY timestamp DESC LIMIT 1",
            )
            .map_err(|e| format!("Failed to prepare query: {}", e))?;

        let result: SqliteResult<Option<String>> = stmt
            .query_row([symbol, &cutoff.to_string()], |row| row.get(0))
            .optional();

        match result {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Failed to query cache: {}", e)),
        }
    }

    async fn cache_data(&self, symbol: &str, data: &str) -> Result<(), String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("Failed to open cache database: {}", e))?;

        conn.execute(
            "INSERT INTO price_cache (symbol, data, timestamp) VALUES (?, ?, ?)",
            [symbol, data, &Utc::now().timestamp().to_string()],
        )
        .map_err(|e| format!("Failed to cache data: {}", e))?;

        Ok(())
    }
}

impl YahooFinanceTool {
    async fn run(&self, input: &str) -> Result<String, String> {
        let days = input
            .parse::<i64>()
            .map_err(|_| "Input must be a valid number of days".to_string())?;

        if days <= 0 {
            return Err("Number of days must be positive".to_string());
        }

        // Check cache first
        if let Some(cached_data) = self.get_cached_data("BTC-USD", days).await? {
            return Ok(cached_data);
        }

        let provider =
            YahooConnector::new().map_err(|e| format!("Failed to create YahooConnector: {}", e))?;
        let end = OffsetDateTime::now_utc();
        let start = end - time::Duration::days(days);

        let resp = provider
            .get_quote_history("BTC-USD", start, end)
            .await
            .map_err(|e| format!("Failed to fetch data: {}", e))?;

        let quotes = resp
            .quotes()
            .map_err(|e| format!("Failed to parse quotes: {}", e))?;

        // Convert quotes to our HistoricalData format
        let historical_data: Vec<HistoricalData> = quotes
            .into_iter()
            .map(|quote| HistoricalData {
                timestamp: quote.timestamp as i64,
                open: quote.open,
                high: quote.high,
                low: quote.low,
                close: quote.close,
                volume: quote.volume as i64,
            })
            .collect();

        let data = serde_json::to_string(&historical_data)
            .map_err(|e| format!("Failed to serialize data: {}", e))?;

        // Cache the fetched data
        self.cache_data("BTC-USD", &data).await?;

        Ok(data)
    }

    fn schema(&self) -> Option<&ToolSchema> {
        Some(&self.schema)
    }
}

#[async_trait]
impl Tool for YahooFinanceTool {
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_yahoo_finance_tool_validation() {
        let tool = YahooFinanceTool::new();

        // Test invalid input
        let result = tool.run("invalid").await;
        assert!(result.is_err());

        // Test negative days
        let result = tool.run("-1").await;
        assert!(result.is_err());

        // Test zero days
        let result = tool.run( "0").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_yahoo_finance_tool_schema() {
        let tool = YahooFinanceTool::new();
        let schema = tool.schema().unwrap();

        assert_eq!(schema.name, "yahoo-finance");
        assert!(schema.description.contains("BTC price"));
        assert_eq!(schema.parameters.len(), 1);
        assert_eq!(schema.parameters[0].name, "days");
        assert_eq!(schema.parameters[0].parameter_type, ParameterType::Integer);
        assert!(schema.parameters[0].required);
    }
}
