use clap::Parser;

use aldebaran::black_scholes::{call_greeks, call_price, put_greeks, put_price};

#[derive(Parser)]
#[command(name = "aldebaran", version, about = "Black-Scholes option pricing & risk (Greeks)")]
struct Cli {
    #[arg(long, default_value = "100.0", help = "Spot price of the underlying")]
    spot: f64,
    #[arg(long, default_value = "105.0", help = "Strike price")]
    strike: f64,
    #[arg(long, default_value = "0.5", help = "Time to maturity in years")]
    time: f64,
    #[arg(long, default_value = "0.03", help = "Risk-free interest rate (e.g. 0.03 = 3%)")]
    rate: f64,
    #[arg(long, default_value = "0.25", help = "Volatility (e.g. 0.25 = 25%)")]
    vol: f64,
    #[arg(long, help = "Only show call prices/greeks")]
    call: bool,
    #[arg(long, help = "Only show put prices/greeks")]
    put: bool,
    #[arg(long, short = 'j', help = "Output as JSON")]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    let show_call = !cli.put || cli.call;
    let show_put = !cli.call || cli.put;

    if cli.json {
        print_json(cli.spot, cli.strike, cli.time, cli.rate, cli.vol, show_call, show_put);
    } else {
        print_human(cli.spot, cli.strike, cli.time, cli.rate, cli.vol, show_call, show_put);
    }
}

fn print_json(spot: f64, strike: f64, time: f64, rate: f64, vol: f64, show_call: bool, show_put: bool) {
    let mut parts = Vec::new();
    if show_call {
        if let Ok(price) = call_price(spot, strike, time, rate, vol) {
            parts.push(format!("\"call_price\": {price}"));
        }
        if let Ok(g) = call_greeks(spot, strike, time, rate, vol) {
            parts.push(format!(
                "\"call_greeks\": {{ \"delta\": {}, \"gamma\": {}, \"vega\": {}, \"theta\": {}, \"rho\": {} }}",
                g.delta, g.gamma, g.vega, g.theta, g.rho
            ));
        }
    }
    if show_put {
        if let Ok(price) = put_price(spot, strike, time, rate, vol) {
            parts.push(format!("\"put_price\": {price}"));
        }
        if let Ok(g) = put_greeks(spot, strike, time, rate, vol) {
            parts.push(format!(
                "\"put_greeks\": {{ \"delta\": {}, \"gamma\": {}, \"vega\": {}, \"theta\": {}, \"rho\": {} }}",
                g.delta, g.gamma, g.vega, g.theta, g.rho
            ));
        }
    }
    println!("{{ {} }}", parts.join(", "));
}

fn print_human(spot: f64, strike: f64, time: f64, rate: f64, vol: f64, show_call: bool, show_put: bool) {
    if show_call {
        match call_price(spot, strike, time, rate, vol) {
            Ok(price) => println!("Call option price: {price:.4}"),
            Err(e) => println!("Error calculating call price: {e}"),
        }
        match call_greeks(spot, strike, time, rate, vol) {
            Ok(g) => println!(
                "Call Greeks — Δ: {:.4}, Γ: {:.4}, ν: {:.4}, Θ: {:.4}, ρ: {:.4}",
                g.delta, g.gamma, g.vega, g.theta, g.rho
            ),
            Err(e) => println!("Error calculating call greeks: {e}"),
        }
    }
    if show_put {
        match put_price(spot, strike, time, rate, vol) {
            Ok(price) => println!("Put option price: {price:.4}"),
            Err(e) => println!("Error calculating put price: {e}"),
        }
        match put_greeks(spot, strike, time, rate, vol) {
            Ok(g) => println!(
                "Put Greeks — Δ: {:.4}, Γ: {:.4}, ν: {:.4}, Θ: {:.4}, ρ: {:.4}",
                g.delta, g.gamma, g.vega, g.theta, g.rho
            ),
            Err(e) => println!("Error calculating put greeks: {e}"),
        }
    }
}
