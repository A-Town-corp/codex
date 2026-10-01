//! API-equivalent value of token totals at the selected model's standard rate.
//!
//! This is not subscription billing. Model switches revalue the entire total.
//! Rates are USD per million tokens, verified 2026-10-01 against
//! https://developers.openai.com/api/docs/pricing (Standard, short context).
//! Tool charges, cache-write premiums, Fast mode, and long-context/regional
//! premiums are excluded because these cumulative counters do not identify them.

use crate::status::format_estimated_usd_micros;
use crate::token_usage::TokenUsage;

pub(crate) fn estimate(model: &str, usage: &TokenUsage) -> Option<String> {
    let (input_rate, cached_rate, output_rate) = match model {
        "gpt-6-astra" => (10.0, 1.0, 50.0),
        "gpt-6.1-sol" => (2.0, 0.1, 10.0),
        "gpt-6-sol" => (2.0, 0.2, 10.0),
        "gpt-6-luna" => (0.1, 0.01, 0.5),
        "gpt-5.6-sol" => (4.0, 0.4, 20.0),
        "gpt-5.6-terra" => (2.0, 0.2, 12.0),
        "gpt-5.6-luna" => (0.2, 0.02, 1.2),
        "gpt-5.5" => (5.0, 0.5, 30.0),
        "gpt-5.4" => (2.5, 0.25, 15.0),
        "gpt-5.4-mini" => (0.75, 0.075, 4.5),
        "gpt-5.4-nano" => (0.2, 0.02, 1.25),
        "gpt-5.3-codex" => (1.75, 0.175, 14.0),
        _ => return None,
    };
    let input = usage.input_tokens.max(0);
    let cached = usage.cached_input_tokens.clamp(0, input);
    // Reasoning tokens are already included in output_tokens.
    let micros = (input - cached) as f64 * input_rate
        + cached as f64 * cached_rate
        + usage.output_tokens.max(0) as f64 * output_rate;
    if micros >= i64::MAX as f64 {
        return None;
    }
    format_estimated_usd_micros(micros.round() as i64)
}
