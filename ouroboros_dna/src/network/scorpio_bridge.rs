use std::str::FromStr;

use ethers_core::types::Address;
use ethers_core::utils::format_units;
use ethers_providers::{Http, Middleware, Provider};
use tokio::runtime::Runtime;

use crate::error::{OuroborosError, Result};

const DEFAULT_PROVIDER: &str = "https://cloudflare-eth.com";

#[derive(Debug)]
pub struct ScorpioBridge {
    provider: Provider<Http>,
    runtime: Runtime,
}

impl Default for ScorpioBridge {
    fn default() -> Self {
        Self::new(DEFAULT_PROVIDER).expect("failed to initialise default ScorpioBridge")
    }
}

impl ScorpioBridge {
    pub fn new(provider_url: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(provider_url)
            .map_err(|e| OuroborosError::Provider(e.to_string()))?;
        let runtime = Runtime::new().map_err(|e| OuroborosError::Provider(e.to_string()))?;
        Ok(Self { provider, runtime })
    }

    pub fn check_seed(&self, address: &str) -> Result<Option<f64>> {
        let parsed = Address::from_str(address)
            .map_err(|e| OuroborosError::Provider(format!("invalid address {address}: {e}")))?;
        let balance = self
            .runtime
            .block_on(self.provider.get_balance(parsed, None))
            .map_err(|e| OuroborosError::Provider(e.to_string()))?;
        let ether =
            format_units(balance, 18).map_err(|e| OuroborosError::Provider(e.to_string()))?;
        let value = ether.parse::<f64>().unwrap_or_default();
        Ok(Some(value))
    }
}
