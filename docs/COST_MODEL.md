# Tokn Cost Model

## Overview

Tokn utilizes a transparent and predictable cost model for LLM API calls. The cost is calculated based on input/output token counts and model-specific pricing tiers.

## Pricing Structure

- **Input Tokens**: Cost per 1,000 tokens.
- **Output Tokens**: Cost per 1,000 tokens.
- **Fixed Costs**: Monthly or per-request fixed fees where applicable.

## Budget Tracking

Tokn provides real-time budget tracking for users and teams.

- **Hard Limits**: Stop requests once the budget is reached.
- **Soft Limits**: Notify when a threshold is reached but allow override.
- **Aggregation**: Costs are aggregated by user, project, and model at 1-minute intervals.

## Implementation Details

- **Currency**: All internal calculations are performed in USD.
- **Rounding**: Results are rounded to 4 decimal places to ensure accuracy.
- **Prometheus Metrics**: Cost data is exposed via `tokn_cost_total` and `tokn_tokens_total` for monitoring.

## Cost Optimization

Tokn's routing engine automatically selects the most cost-effective model that meets latency and quality requirements based on historical performance data.
