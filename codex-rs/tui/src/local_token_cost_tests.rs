use super::estimate;
use crate::token_usage::TokenUsage;
use pretty_assertions::assert_eq;

#[test]
fn values_cached_input_and_output_for_all_supported_models() {
    let usage = TokenUsage {
        input_tokens: 100_000,
        cached_input_tokens: 80_000,
        output_tokens: 10_000,
        reasoning_output_tokens: 5_000,
        total_tokens: 110_000,
    };
    for (model, expected) in [
        ("gpt-6.2-astra", "~$0.59"),
        ("gpt-6.2-sol", "~$0.15"),
        ("gpt-6-astra", "~$0.78"),
        ("gpt-6.1-sol", "~$0.15"),
        ("gpt-6.1-terra", "~$0.07"),
        ("gpt-6-sol", "~$0.16"),
        ("gpt-6-luna", "~$0.0078"),
        ("gpt-5.6-sol", "~$0.31"),
        ("gpt-5.6-terra", "~$0.18"),
        ("gpt-5.6-luna", "~$0.02"),
        ("gpt-5.5", "~$0.44"),
        ("gpt-5.4", "~$0.22"),
        ("gpt-5.4-mini", "~$0.07"),
        ("gpt-5.4-nano", "~$0.02"),
        ("gpt-5.3-codex", "~$0.19"),
    ] {
        assert_eq!(
            estimate(model, &usage),
            Some(expected.to_string()),
            "{model}"
        );
    }
}

#[test]
fn clamps_invalid_counters_and_preserves_small_amounts() {
    for (input_tokens, cached_input_tokens, output_tokens, expected) in [
        (-1, -10, -4, "~$0.00"),
        (100, 1_000, 0, "~$0.000010"),
        (1, 0, 0, "~$0.000002"),
    ] {
        let usage = TokenUsage {
            input_tokens,
            cached_input_tokens,
            output_tokens,
            ..TokenUsage::default()
        };
        assert_eq!(estimate("gpt-6.1-sol", &usage), Some(expected.to_string()));
    }
}

#[test]
fn does_not_double_count_reasoning_tokens() {
    let usage = TokenUsage {
        output_tokens: 10_000,
        reasoning_output_tokens: 10_000,
        ..TokenUsage::default()
    };
    assert_eq!(estimate("gpt-6.1-sol", &usage), Some("~$0.10".to_string()));
}

#[test]
fn refuses_unknown_models_and_unrepresentable_totals() {
    assert_eq!(
        estimate("unknown-local-model", &TokenUsage::default()),
        None
    );
    let usage = TokenUsage {
        input_tokens: i64::MAX,
        ..TokenUsage::default()
    };
    assert_eq!(estimate("gpt-6-astra", &usage), None);
}
