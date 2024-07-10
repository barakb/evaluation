use clap::{Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(version, name = "Evaluation", about = "Evaluate stock based on P/E and growth")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Ask for evaluation of a specific P/E and a growth rate
    #[command(
        arg_required_else_help = true,
        about = "Ask for evaluation of a specific P/E and a growth rate",
        arg_required_else_help = false
    )]
    Ask {
        /// The Price to Earnings ratio of the stock
        #[arg(short, long, default_value = "8", required = false)]
        pe: u16,
        /// The growth rate of the stock in percentage
        #[arg(short, long, default_value = "8", required = false)]
        growth: u16,
        /// the number of years to consider for evaluation
        #[arg(short, long, default_value = "5", required = false)]
        years: u8,
    },
}
