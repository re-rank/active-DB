// Library interface for activedb-cli to enable testing
use clap::Subcommand;

pub mod cleanup;
pub mod commands;
pub mod config;
pub mod docker;
pub mod errors;
pub mod github_issue;
pub mod metrics_sender;
pub mod output;
pub mod port;
pub mod project;
pub mod prompts;
pub mod sse_client;
pub mod update;
pub mod utils;

#[derive(Subcommand)]
pub enum AuthAction {
    /// Login to ActiveDB cloud
    Login,
    /// Logout from ActiveDB cloud
    Logout,
    /// Rotate a cluster API key
    CreateKey {
        /// Cluster ID
        cluster: String,
    },
}

#[derive(Subcommand)]
pub enum MetricsAction {
    /// Enable metrics collection
    Full,
    /// Disable metrics collection
    Basic,
    /// Disable metrics collection
    Off,
    /// Show metrics status
    Status,
}

#[derive(Subcommand)]
pub enum DashboardAction {
    /// Start the dashboard
    Start {
        /// Instance to connect to (from activedb.toml)
        instance: Option<String>,

        /// Port to run dashboard on
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// ActiveDB host to connect to (e.g., localhost). Bypasses project config.
        #[arg(long)]
        host: Option<String>,

        /// ActiveDB port to connect to. Used with --host.
        #[arg(long, default_value = "6969")]
        activedb_port: u16,

        /// Run dashboard in foreground with logs
        #[arg(long)]
        attach: bool,

        /// Restart if dashboard is already running
        #[arg(long)]
        restart: bool,
    },
    /// Stop the dashboard
    Stop,
    /// Show dashboard status
    Status,
}

#[derive(Subcommand)]
pub enum CloudDeploymentTypeCommand {
    /// Initialize ActiveDB Cloud deployment
    #[command(name = "cloud")]
    ActiveDB {
        /// Region for ActiveDB cloud instance (default: us-east-1)
        #[arg(long, default_value = "us-east-1")]
        region: Option<String>,

        /// Instance name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Initialize ECR deployment
    Ecr {
        /// Instance name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Initialize Fly.io deployment
    Fly {
        /// Authentication type
        #[arg(long, default_value = "cli")]
        auth: String,

        /// volume size
        #[arg(long, default_value = "20")]
        volume_size: u16,

        /// vm size
        #[arg(long, default_value = "shared-cpu-4x")]
        vm_size: String,

        /// privacy
        #[arg(long, default_value = "false")]
        private: bool,

        /// Instance name
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Initialize Local deployment
    Local {
        /// Instance name
        #[arg(short, long)]
        name: Option<String>,
    },
}

impl CloudDeploymentTypeCommand {
    pub fn name(&self) -> Option<String> {
        match self {
            CloudDeploymentTypeCommand::ActiveDB { name, .. } => name.clone(),
            CloudDeploymentTypeCommand::Ecr { name } => name.clone(),
            CloudDeploymentTypeCommand::Fly { name, .. } => name.clone(),
            CloudDeploymentTypeCommand::Local { name } => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests;
