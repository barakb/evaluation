use clap::Parser;
use tabled::{
    builder::Builder,
    settings::Style,
};

use crate::cli::{Cli, Commands};

mod cli;

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Ask { pe, growth, years } => {
            ask(pe, growth, years)
        }
    }
}

fn ask(pe: u16, growth: u16, years: u8) {
    println!("pe:{} ({}%), growth:{}%, years:{}", pe, 100.0 / pe as f64, growth, years);
    let mut builder = Builder::default();
    let header = vec!["Year", "Value"];
    builder.push_record(header);

    let pe_growth = 1.0 + (1.0 / pe as f64);
    let yearly_growth = 1.0 + (growth as f64 / 100.0);
    for year in 1..years + 1 {
        let total_growth = pe_growth * (yearly_growth.powi(year as i32));
        let current_value = to_percent(total_growth);
        builder.push_record([year.to_string(), current_value.to_string()]);
    }

    let table = builder.build()
        .with(Style::modern())
        .to_string();

    println!("{}", table);
}

fn to_percent(value: f64) -> String {
    format!("{:.1}%", (value - 1.0) * 100.0)
}
