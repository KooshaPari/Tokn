# Comparison Matrix

## Feature Comparison

This document compares **tokenledger** with similar tools in the token tracking and AI cost management space.

| Repository | Purpose | Key Features | Language/Framework | Maturity | Comparison |
|------------|---------|--------------|-------------------|----------|------------|
| **tokenledger (this repo)** | Token management & pricing governance | Multi-provider, Cost optimization, CLI reporting | Rust | Stable | Enterprise token governance |
| [Helicone](https://github.com/Helicone/helicone) | LLM observability | Logging, Analytics, Caching | TypeScript | Stable | Open source observability |
| [Portkey](https://github.com/PortKey-AI/portkey) | LLM gateway | Observability, Routing, Analytics | Python | Stable | Enterprise gateway |
| [Braintrust](https://github.com/braintrustdata/braintrust) | Evaluation platform | Traces, Evals, Analytics | Python | Stable | Evaluation & tracing |
| [PromptLayer](https://github.com/MagnivOrg/promptlayer) | Prompt management | Versioning, Analytics, Metadata | Python | Stable | Prompt tracking |
| [Weights & Biases](https://github.com/wandb/wandb) | ML experiment tracking | Experiments, Logging, Artifacts | Python | Stable | General experiment tracking |

## Detailed Feature Comparison

### Token & Cost Management

| Feature | tokenledger | Helicone | Portkey | Braintrust |
|---------|-------------|----------|---------|------------|
| Token Tracking | ✅ | ✅ | ✅ | ✅ |
| Cost Calculation | ✅ | ✅ | ✅ | ✅ |
| Multi-Provider | ✅ (OpenAI, Anthropic) | ✅ | ✅ | ✅ |
| Optimization | ✅ | ❌ | ✅ | ❌ |
| Pricing Governance | ✅ | ❌ | ❌ | ❌ |

### CLI & Integration

| Feature | tokenledger | Helicone | Portkey | PromptLayer |
|---------|-------------|----------|---------|-------------|
| CLI Tool | ✅ | ❌ | ❌ | ✅ |
| Python SDK | ✅ | ✅ | ✅ | ✅ |
| REST API | ❌ | ✅ | ✅ | ✅ |
| thegent Integration | ✅ | ❌ | ❌ | ❌ |

### Code Quality

| Metric | tokenledger | Threshold |
|--------|-------------|-----------|
| Test coverage | >= 80% | tarpaulin |
| Security findings | 0 high/critical | cargo-audit |
| Clippy warnings | 0 | CI gate |
| Max function lines | 40 | Style guide |

## Unique Value Proposition

tokenledger provides:

1. **Pricing Governance**: Enterprise-grade cost control and optimization
2. **Rust Performance**: High-performance token counting and cost calculation
3. **Multi-Provider**: Unified tracking across OpenAI and Anthropic
4. **thegent Integration**: Native integration with thegent agent framework

## Commands

```bash
# Token ledger CLI
cargo run -- report
cargo run -- costs
cargo run -- optimize
```

## References

- Helicone: [ Helicone/helicone](https://github.com/Helicone/helicone)
- Portkey: [PortKey-AI/portkey](https://github.com/PortKey-AI/portkey)
- Braintrust: [braintrustdata/braintrust](https://github.com/braintrustdata/braintrust)
