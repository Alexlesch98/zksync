//! Module with different scenarios for a `loadtest`.
//! A scenario is basically is a behavior policy for sending the transactions.
//! A simplest scenario will be: "get a bunch of accounts and just spawn a lot of transfer
//! operations between them".

// Built-in import
use std::{path::PathBuf, str::FromStr, sync::Arc};
// External uses
use tokio::runtime::Runtime;
// Workspace uses
use zksync::Provider;
use zksync_config::ConfigurationOptions;
// Local uses
use super::tps_counter::TPSCounter;
use crate::monitor::Monitor;

pub(crate) mod configs;
mod deprecated;
mod real_life;
mod utils;

pub type Scenario = Box<dyn Fn(ScenarioContext)>;

/// Supported scenario types.
#[derive(Debug, Clone, Copy)]
pub enum ScenarioType {
    /// Measure the outgoing TPS (ZKSync node mempool acceptance throughput).
    OutgoingTps,
    /// Measure the TPS for transactions execution (not including verifying).
    ExecutionTps,
    /// Run the real-life scenario.
    RealLife,
}

impl ScenarioType {
    /// Returns the scenario function given its type.
    pub fn into_scenario(self) -> Scenario {
        match self {
            Self::OutgoingTps => Box::new(deprecated::outgoing_tps::run_scenario),
            Self::ExecutionTps => Box::new(deprecated::execution_tps::run_scenario),
            Self::RealLife => Box::new(real_life::run_scenario),
        }
    }
}

impl FromStr for ScenarioType {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scenario = match s {
            "outgoing" | "outgoing_tps" => Self::OutgoingTps,
            "execution" | "execution_tps" => Self::ExecutionTps,
            "reallife" | "real-life" | "real_life" => Self::RealLife,
            other => {
                failure::bail!(
                    "Unknown scenario type '{}'. \
                     Available options are: \
                     'outgoing_tps', 'execution_tps', 'real_life', \
                     'api_test'",
                    other
                );
            }
        };

        Ok(scenario)
    }
}

#[derive(Debug)]
pub struct ScenarioContext {
    pub monitor: Monitor,
    pub options: ConfigurationOptions,
    pub config_path: PathBuf,
    pub tps_counter: Arc<TPSCounter>,
    pub rt: Runtime,
}

impl ScenarioContext {
    pub fn new(
        provider: Provider,
        options: ConfigurationOptions,
        config_path: PathBuf,
        rt: Runtime,
    ) -> Self {
        let tps_counter = Arc::new(TPSCounter::default());

        Self {
            monitor: Monitor { provider },
            options,
            config_path,
            tps_counter,
            rt,
        }
    }
}
