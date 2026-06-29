use std::f64::consts::PI;

// Abramowitz & Stegun approximation constants for normal CDF
const A1: f64 = 0.319381530;
const A2: f64 = -0.356563782;
const A3: f64 = 1.781477937;
const A4: f64 = -1.821255978;
const A5: f64 = 1.330274429;
const P: f64 = 0.2316419;

/// Standard normal probability density function
fn norm_pdf(x: f64) -> f64 {
    (1.0 / (2.0 * PI).sqrt()) * (-x * x / 2.0).exp()
}

/// Standard normal cumulative distribution function
fn norm_cdf(x: f64) -> f64 {
    if x.is_nan() || x.is_infinite() {
        return if x.is_sign_negative() { 0.0 } else { 1.0 };
    }
    let x_abs = x.abs();
    let k = 1.0 / (1.0 + P * x_abs);
    let phi = norm_pdf(x_abs);
    let cdf = 1.0 - phi * (A1 * k + A2 * k.powi(2) + A3 * k.powi(3) + A4 * k.powi(4) + A5 * k.powi(5));
    if x.is_sign_negative() { 1.0 - cdf } else { cdf }
}

/// Calculates d1 and d2 from the Black-Scholes model.
fn calculate_d1_d2(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> Result<(f64, f64), String> {
    if s <= 0.0 {
        return Err("Spot price must be positive".into());
    }
    if k <= 0.0 {
        return Err("Strike price must be positive".into());
    }
    if t <= 0.0 {
        return Err("Time to maturity must be positive".into());
    }
    if sigma <= 0.0 {
        return Err("Volatility must be positive".into());
    }
    let sigma_sqrt_t = sigma * t.sqrt();
    let d1 = ((s / k).ln() + (r + sigma * sigma / 2.0) * t) / sigma_sqrt_t;
    let d2 = d1 - sigma_sqrt_t;
    Ok((d1, d2))
}

/// Black-Scholes price for a European call option.
pub fn call_price(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> Result<f64, String> {
    let (d1, d2) = calculate_d1_d2(s, k, t, r, sigma)?;
    Ok(s * norm_cdf(d1) - k * (-r * t).exp() * norm_cdf(d2))
}

/// Black-Scholes price for a European put option.
pub fn put_price(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> Result<f64, String> {
    let (d1, d2) = calculate_d1_d2(s, k, t, r, sigma)?;
    Ok(k * (-r * t).exp() * norm_cdf(-d2) - s * norm_cdf(-d1))
}

/// Greeks for a European call option.
pub fn call_greeks(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> Result<CallGreeks, String> {
    let (d1, d2) = calculate_d1_d2(s, k, t, r, sigma)?;
    let phi_d1 = norm_pdf(d1);
    let nd2 = norm_cdf(d2);
    let discount = (-r * t).exp();

    let delta = norm_cdf(d1);
    let gamma = phi_d1 / (s * sigma * t.sqrt());
    let vega = s * phi_d1 * t.sqrt();
    let theta = -(s * phi_d1 * sigma) / (2.0 * t.sqrt()) - r * k * discount * nd2;
    let rho = k * t * discount * nd2;

    Ok(CallGreeks { delta, gamma, vega, theta, rho })
}

/// Greeks for a European put option.
pub fn put_greeks(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> Result<PutGreeks, String> {
    let (d1, d2) = calculate_d1_d2(s, k, t, r, sigma)?;
    let phi_d1 = norm_pdf(d1);
    let n_minus_d2 = norm_cdf(-d2);
    let discount = (-r * t).exp();

    let delta = norm_cdf(d1) - 1.0;
    let gamma = phi_d1 / (s * sigma * t.sqrt());
    let vega = s * phi_d1 * t.sqrt();
    let theta = -(s * phi_d1 * sigma) / (2.0 * t.sqrt()) + r * k * discount * n_minus_d2;
    let rho = -k * t * discount * n_minus_d2;

    Ok(PutGreeks { delta, gamma, vega, theta, rho })
}

macro_rules! greeks_struct {
    ($name:ident, $option_type:literal) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name {
            /// Sensitivity to underlying price changes
            pub delta: f64,
            /// Sensitivity to delta changes (curvature risk)
            pub gamma: f64,
            /// Sensitivity to volatility changes (per 1% vol change, divide by 100)
            pub vega: f64,
            /// Sensitivity to time decay (per day, divide by 365)
            pub theta: f64,
            /// Sensitivity to interest rate changes
            pub rho: f64,
        }
    };
}

greeks_struct!(CallGreeks, "call");
greeks_struct!(PutGreeks, "put");

#[cfg(test)]
mod tests {
    use super::*;

    // Known values: S=100, K=100, T=1, r=0.05, sigma=0.2
    // d1 = (ln(1) + (0.05 + 0.02)) / 0.2 = 0.07/0.2 = 0.35
    // d2 = 0.35 - 0.2 = 0.15
    const S: f64 = 100.0;
    const K: f64 = 100.0;
    const T: f64 = 1.0;
    const R: f64 = 0.05;
    const SIGMA: f64 = 0.2;

    fn approx_eq(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() < tol, "expected {a}, got {b}");
    }

    #[test]
    fn test_norm_cdf() {
        approx_eq(norm_cdf(0.0), 0.5, 1e-7);
        approx_eq(norm_cdf(1.0), 0.841344746, 1e-6);
        approx_eq(norm_cdf(-1.0), 0.158655254, 1e-6);
        approx_eq(norm_cdf(1.96), 0.975002105, 1e-6);
    }

    #[test]
    fn test_norm_cdf_extreme() {
        approx_eq(norm_cdf(f64::NEG_INFINITY), 0.0, 1e-15);
        approx_eq(norm_cdf(f64::INFINITY), 1.0, 1e-15);
    }

    #[test]
    fn test_atm_call() {
        let price = call_price(S, K, T, R, SIGMA).unwrap();
        // ATM call with these params ~10.45
        approx_eq(price, 10.4506, 1e-2);
    }

    #[test]
    fn test_atm_put() {
        let price = put_price(S, K, T, R, SIGMA).unwrap();
        // ATM put ~5.57 (put-call parity: C - P = S - K*e^(-rT))
        approx_eq(price, 5.5735, 1e-2);
    }

    #[test]
    fn test_put_call_parity() {
        let call = call_price(S, K, T, R, SIGMA).unwrap();
        let put = put_price(S, K, T, R, SIGMA).unwrap();
        let lhs = call - put;
        let rhs = S - K * (-R * T).exp();
        approx_eq(lhs, rhs, 1e-10);
    }

    #[test]
    fn test_itm_call() {
        let price = call_price(110.0, 100.0, 1.0, 0.05, 0.2).unwrap();
        approx_eq(price, 17.6629, 1e-2);
    }

    #[test]
    fn test_otm_call() {
        let price = call_price(90.0, 100.0, 1.0, 0.05, 0.2).unwrap();
        approx_eq(price, 5.0912, 1e-2);
    }

    #[test]
    fn test_invalid_inputs() {
        assert!(call_price(-1.0, 100.0, 1.0, 0.05, 0.2).is_err());
        assert!(call_price(100.0, 0.0, 1.0, 0.05, 0.2).is_err());
        assert!(call_price(100.0, 100.0, 0.0, 0.05, 0.2).is_err());
        assert!(call_price(100.0, 100.0, 1.0, 0.05, -0.1).is_err());
    }

    #[test]
    fn test_call_delta_range() {
        let g = call_greeks(S, K, T, R, SIGMA).unwrap();
        assert!(g.delta > 0.0 && g.delta < 1.0);
    }

    #[test]
    fn test_put_delta_range() {
        let g = put_greeks(S, K, T, R, SIGMA).unwrap();
        assert!(g.delta > -1.0 && g.delta < 0.0);
    }

    #[test]
    fn test_gamma_positive() {
        let cg = call_greeks(S, K, T, R, SIGMA).unwrap();
        let pg = put_greeks(S, K, T, R, SIGMA).unwrap();
        assert!(cg.gamma > 0.0);
        assert!((cg.gamma - pg.gamma).abs() < 1e-15);
    }

    #[test]
    fn test_vega_positive() {
        let cg = call_greeks(S, K, T, R, SIGMA).unwrap();
        let pg = put_greeks(S, K, T, R, SIGMA).unwrap();
        assert!(cg.vega > 0.0);
        assert!((cg.vega - pg.vega).abs() < 1e-15);
    }

    #[test]
    fn test_theta_negative() {
        let cg = call_greeks(S, K, T, R, SIGMA).unwrap();
        assert!(cg.theta < 0.0);
    }
}
