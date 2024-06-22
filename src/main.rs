use std::collections::HashMap;
use tabled::{Table, Tabled, settings::Style};


#[derive(Tabled)]
struct Stock {
    pe: u32,
    #[tabled(rename = "yield")]
    yearly_yield: String,
    #[tabled(rename = "doubled 3%")]
    doubled_3_percent_growth: String,
    #[tabled(rename = "doubled 5%")]
    doubled_5_percent_growth: String,
    #[tabled(rename = "doubled 7%")]
    doubled_7_percent_growth: String,
    #[tabled(rename = "doubled 10%")]
    doubled_10_percent_growth: String,
    #[tabled(rename = "doubled 15%")]
    doubled_15_percent_growth: String,
    #[tabled(rename = "doubled 20%")]
    doubled_20_percent_growth: String,
    #[tabled(rename = "doubled 25%")]
    doubled_25_percent_growth: String,
}

#[derive(Tabled)]
#[derive(Debug, Clone)]
struct Buy {
    pe: u32,
    #[tabled(rename = "growth")]
    yearly_growth: String,
}

fn main() {

    // let prices_per_earnings : Vec<u32>=  (1..21).collect();
    let prices_per_earnings = (1..51).into_iter().map(pe_to_stock).collect::<Vec<Stock>>();
    let table = Table::new(&prices_per_earnings).with(Style::modern()).to_string();
    println!("all values");
    println!("{}", table);
    let buy = prices_per_earnings.iter().filter_map(to_buy).collect::<Vec<Buy>>();
    println!("Buy values");
    let table = Table::new(&buy).with(Style::modern()).to_string();
    println!("{}", table);
    let m: HashMap<String, Buy> = buy.clone().iter().fold(HashMap::new(), buy_by_yield);
    let mut buy_compact = m
        .values().collect::<Vec<&Buy>>();
    buy_compact.sort_by_key(|buy| buy.pe);
    println!("Buy values compact");
    let table = Table::new(&buy_compact).with(Style::modern()).to_string();
    println!("{}", table);

}

fn buy_by_yield(mut m: HashMap<String, Buy>, buy: &Buy) -> HashMap<String, Buy> {
    if m.contains_key(&(buy.yearly_growth)){
        m
    }else{
        let value = buy.clone();
        let key = (&buy.yearly_growth).clone();
        m.insert(key, value);
        m
    }
}

fn to_buy(stock: &Stock) -> Option<Buy> {
    if stock.pe < 6 {
        Some(Buy { pe: stock.pe, yearly_growth: "0".to_string() })
    } else if stock.doubled_3_percent_growth.parse().unwrap_or(10.0) < 6.0 {
        Some(Buy { pe: stock.pe, yearly_growth: "3".to_string() })
    } else if stock.doubled_5_percent_growth.parse().unwrap_or(10.0) < 6.0 {
        Some(Buy { pe: stock.pe, yearly_growth: "5".to_string() })
    } else if stock.doubled_10_percent_growth.parse().unwrap_or(10.0) < 6.0 {
        Some(Buy { pe: stock.pe, yearly_growth: "10".to_string() })
    } else if stock.doubled_15_percent_growth.parse().unwrap_or(10.0) < 6.0 {
        Some(Buy { pe: stock.pe, yearly_growth: "15".to_string() })
    } else if stock.doubled_20_percent_growth.parse().unwrap_or(10.0) < 6.0 {
        Some(Buy { pe: stock.pe, yearly_growth: "20".to_string() })
        } else if stock.doubled_25_percent_growth.parse().unwrap_or(10.0) < 6.0 {
        Some(Buy { pe: stock.pe, yearly_growth: "25".to_string() })
    } else {
        None
    }
}

fn pe_to_stock(pe: u32) -> Stock {
    let percent = 1.0 / (pe as f64) * 100.0;
    let doubled_3_percent_growth = compute_doubled(1.0 / (pe as f64), 1.03);
    let doubled_5_percent_growth = compute_doubled(1.0 / (pe as f64), 1.05);
    let doubled_7_percent_growth = compute_doubled(1.0 / (pe as f64), 1.07);
    let doubled_10_percent_growth = compute_doubled(1.0 / (pe as f64), 1.1);
    let doubled_15_percent_growth = compute_doubled(1.0 / (pe as f64), 1.15);
    let doubled_20_percent_growth = compute_doubled(1.0 / (pe as f64), 1.2);
    let doubled_25_percent_growth = compute_doubled(1.0 / (pe as f64), 1.25);
    Stock {
        pe,
        yearly_yield: format!("{:.1}%", percent),
        doubled_3_percent_growth,
        doubled_5_percent_growth,
        doubled_7_percent_growth,
        doubled_10_percent_growth,
        doubled_15_percent_growth,
        doubled_20_percent_growth,
        doubled_25_percent_growth,
    }
}

fn compute_doubled(yearly_percent: f64, growth: f64) -> String {
    let mut value = 1.0;
    let yearly_percent = 1.0 + yearly_percent;
    for year in 1.. {
        value = value * yearly_percent * growth;
        if 2.0 <= value {
            return year.to_string();
        }
    }
    "Never".to_string()
}
