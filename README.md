# caliber-echo

[![License: AGPL-3.0](https://img.shields.io/github/license/dnacenta/caliber-echo)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange)](https://rustup.rs/)

Operational self-model and outcome tracking for [pulse-null](https://github.com/dnacenta/pulse-null) entities.

Records what was attempted, what happened, and how predictions compared to reality. Every scheduled task and intent execution produces an `OutcomeRecord` with task type, domain, outcome classification, and token usage. Over time, this builds a data-backed picture of what the entity is good at, where it struggles, and how it's changing.

## Features

- **Outcome tracking**: Records task type, domain, outcome (success/partial/failed/surprising), and token usage
- **Inference**: Automatically classifies task type and domain from task metadata
- **Rolling window**: Configurable max outcomes with oldest-first eviction
- **Render**: Produces a prompt-injectable summary with success rates, domain breakdowns, and failure patterns
- **Health checks**: Reports status based on CALIBER.md and caliber directory presence

## Usage

caliber-echo is used as a dependency of pulse-null. Configure in your entity config:

```toml
[plugins.caliber-echo]
docs_dir = "/path/to/entity"
```

Outcome recording happens automatically after every scheduled task and intent execution.

## License

AGPL-3.0 — see [LICENSE](LICENSE).
