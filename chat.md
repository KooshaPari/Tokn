Next
Thought for 4m 24s
Answer now
Helios v1.1 Fillable Checklist Pack

Copy/paste templates for Notion / Jira / Linear / Google Docs
Goal: make your scenario worksheet executable by anyone with minimal interpretation.

1. Master Test Run Sheet (one per run)

Title: Helios v1.1 Routing & Patch DAG Validation — Run #___
Date: ___
Operator: ___
Snapshot versions:

OfferSnapshot: ___

EconomicsSnapshot: ___

TelemetrySnapshot: ___

QualitySnapshot: ___

Environment: ☐ Local sim ☐ Staging ☐ Prod shadow
Budget mode: ☐ Green ☐ Amber ☐ Red
Notes / known incidents: ___

Pass criteria for run:

☐ Zero invariant violations

☐ ≥ ___ scenarios executed

☐ ≥ ___% scenarios passed

☐ Any failures triaged with actionable root cause

2. Scenario Execution Ticket Template (one per scenario)

Title: [Helios] Scenario S-__ — short name
Type: Test / Validation
Priority: ☐ P0 ☐ P1 ☐ P2
Owner: ___
Status: ☐ Not started ☐ Running ☐ Blocked ☐ Passed ☐ Failed ☐ Waived

A) Scenario definition

Scenario ID: S-__
Role: ___
Workflow: ☐ Route-only ☐ Patch DAG (Reason → Apply → Validate)
Request summary:

needsTools: ☐ yes ☐ no

needsJson: ☐ yes ☐ no

requirePromptContract: ☐ none ☐ apply_v1

minContextTokens: ___

minQuality: ___

maxCostUsd: ___

tokenEstimate: tokIn ___ / tokOut ___

B) Knobs set (fill exact values)
Economics knobs

global.budgetMonthlyUsd: 600

global.budgetRemainingUsd: ___

global.budgetShadow: ___

Copilot plan (if relevant):

unitsRemaining: ___ / ___

unit multipliers used: ☐ yes confirmed ☐ not applicable

Cerebras Code plan (if relevant):

tokensRemainingToday: ___ / ___

tpmRemaining: ___ (optional)

rpmRemaining: ___ (optional)

Telemetry knobs (key offers)

Provide 3–6 offers you expect to be relevant and fill their p95 metrics:

offerId class queueP95 ttftP95 itlP95 tpsP50 errorRate schemaAdh

---

---

---

Quality knobs (role-specific)
offerId class qualityScore(role) confidence(role) effective = score*conf

---

---

C) Expected outcome (must be explicit)

Expected chosen class: ___
Expected forbidden classes (must not appear in candidates): ___
Expected fallback chain template: ___
(e.g., “AP → AP → escalate reasoner”, or “FQ → FQ → MQ → C0”)

D) Actual outcome (fill after running)

Chosen offerId: ___
Chosen class: ___
Fallback chain (offerIds):

Pareto set size: ___
Pareto set classes present: ___

Candidate set quick check

☐ Forbidden classes absent from candidates

☐ Hard constraints enforced (no violations)

☐ MinQuality enforced

☐ MaxCost enforced

☐ RoleOnly / promptContract enforced (if applicable)

E) Diff / verdict

Result: ☐ Passed ☐ Failed ☐ Waived
If Failed, what differed?

☐ wrong chosen class

☐ forbidden class appeared

☐ hard constraint violated

☐ fallback chain wrong

☐ economics/shadow not applied

☐ speed scoring not responsive

☐ quality scoring unexpected

Notes / suspected root cause: ___
Artifacts attached:

☐ Route trace JSON

☐ Logs (TTFT/ITL)

☐ Validation output

☐ Patch outputs (if Patch DAG)

3. Invariant Violation Ticket (for any P0 issue)

Title: [Helios][P0] Invariant violation — short
Invariant ID: (e.g., INV-02 Apply role isolation)
Severity: ☐ P0 Blocker
Environment: ___
When observed: timestamp ___
Snapshot versions: offer ___ / econ ___ / tel ___ / qual ___

What happened

Request: role ___, constraints ___
Observed: ___
Expected: ___

Evidence

Route trace: (attach)

Logs: (attach)

Provider responses: (attach)

Immediate action

☐ Disable offer(s)

☐ Flip role policy to disallow

☐ Raise minQuality / tighten constraints

☐ Adjust volatility/throttle penalties

Root cause hypothesis
Fix plan 4) Patch DAG-Specific Checklist (attach to Patch DAG scenarios)

Patch DAG scenario addendum

Patch generator stage

☐ Output format adhered (JSON {update:...})

☐ Update is minimal (not full rewrite)

☐ Update references correct file

☐ Update does not include destructive instructions unless requested

Apply stage

☐ Apply offer selected is AP class

☐ Prompt contract used: apply_v1

☐ Output contains only updated file (no explanation)

☐ Output is non-empty and parses as code

☐ File-level diff appears reasonable (sanity check)

Validation stage (if enabled)

☐ Tests/lint/build executed

☐ If failed: impacted files identified

☐ Escalation scope limited to impacted files

Escalation handling

☐ Triggered only on failure conditions

☐ No infinite retries

☐ Cost escalation bounded by policy

☐ Final output correct / rollback possible

5. Summary Report Template (after the run)

Title: Helios v1.1 Validation Summary — Run #___

Coverage

Scenarios planned: ___

Scenarios executed: ___

Passed: ___

Failed: ___

Waived: ___

Key failures (top 5)

S-__ : ___

S-__ : ___

…

Routing distribution notes

Unexpected over-selection: ___

Unexpected under-selection: ___

VF appearances: ___

AP usage: ___

Economics notes

Copilot units behavior: ___

Cerebras daily cap behavior: ___

BudgetShadow behavior: ___

Speed/telemetry notes

TTFT/ITL anomalies: ___

Queue spikes: ___

Flaky offers detected: ___

Actions

☐ Promote offers: ___

☐ Keep in canary: ___

☐ Disable offers: ___

☐ Policy changes needed: ___

6. Ready-to-use “Scenario list” table (paste into Notion)
   Scenario Role Workflow Expected Class Priority Status Owner
   S-01 Apply never for reasoner code_reasoner Route FQ P0 ☐ ___
   S-02 Apply role only AP code_apply_patch Route/DAG AP P0 ☐ ___
   S-03 VF never primary critical agent_workflow Route FQ P0 ☐ ___
   S-04 C0 wins patch gen code_patch_generator Route C0 P1 ☐ ___
   S-05 CU multipliers under scarcity code_patch_generator Route CU low-mult P1 ☐ ___
   S-08 UB early-day scaffold code_scaffold_fast Route UB P1 ☐ ___
   S-09 UB late-day avoided code_scaffold_fast Route UF/MQ/SH P1 ☐ ___
   S-23 Single-file DAG success small_transform(file) DAG patch:C0/MQ + apply:AP P0 ☐ ___
   S-24 Apply failure escalates file apply_patch DAG escalate reasoner P0 ☐ ___

(Add the rest of S-06…S-26 as needed.)

If you want the “next-next” artifact

I can produce a gating rubric you can enforce automatically (promotion/demotion rules), including:

exact thresholds by role and offer class

cooldown periods

“max % traffic” ramps

conditions that force automatic rollback

Yes
Thought for 5m 34s
Answer now
Helios v1.1 Gating Rubric

Promotion / Demotion / Cooldowns / Traffic Ramps / Auto-Rollback
(Designed to be automated from your snapshots + telemetry; no model names required.)

This is the set of rules that keeps your router from turning into “whatever looked cheap today,” and stops “free/preview/ultra-fast” endpoints from quietly wrecking reliability.

1. Offer lifecycle states

Every offer in the OfferSnapshot has a lifecycleState plus allowedRoles and maxTrafficShareByRole.

States

DISABLED
Not eligible for routing. Used for permanently bad offers or manual kill switch.

CANARY
Eligible only for a limited set of roles and limited traffic share. Must earn promotion.

ACTIVE
Fully eligible per the routing policy matrix (but still constrained by role allowlists).

DEGRADED
Still eligible, but receives a large penalty and traffic cap. Used when it’s “kinda working” but unhealthy (rate limits, queues, partial outage).

SUSPENDED
Temporarily removed due to circuit breaker. Auto-retry after cooldown.

Tell-it-like-it-is:
If you don’t have these states, you will eventually ship a “fast/free” endpoint that dominates, then you’ll spend a week chasing ghost regressions.

2. Metrics that gates are allowed to use

Everything below must be available from telemetry aggregation and economics snapshots.

Reliability metrics (per offer + role, over window W)

HardFailRate(W) = (timeouts + provider_5xx + schema_failure + tool_failure) / total

RateLimitRate(W) = 429 / total

BadRequestRate(W) = 400-class / total (often “your prompt broke their contract”)

SuccessRate(W) = 1 − (HardFailRate + RateLimitRate + BadRequestRate)

Output correctness / adherence (role-specific)

SchemaAdherence(W)

patch generator: valid JSON {update: ...} rate

tool roles: tool-call schema validity rate

apply role: “output is file-only” + non-empty + contract compliant

ApplySuccess(W) (for code_apply_patch)

“apply produced a usable file” / total applies

ValidatorPass(W) (when validator enabled)

tests/lint/build pass rate

EscalationRate(W)

% of patch DAG runs that require reasoner fallback (file-scoped)

Speed profile (per offer + role)

TTFT_p95(W)

ITL_p95(W) (or TPOT proxy)

Queue_p95(W) (if measurable)

SpeedScore_p95(W) (your role-specific composite)

Economics / quota health

BudgetShadow (global)

PlanShadow (per plan)

DailyQuotaHealth (for daily bucket plans):
tokensRemainingToday / expectedRemainingToday

UnitQuotaHealth (for Copilot-like unit plans):
unitsRemaining / expectedRemaining

ThrottlePressure: derived from observed 429s + remaining TPM/RPM (if known)

3. Windowing rules (so gates aren’t noisy)

Use multiple windows; promotion needs stability, demotion needs speed.

Standard windows

W5m: last 5 minutes (fast circuit breaker)

W1h: last 1 hour (promotion + demotion confirmation)

W24h: last 24 hours (promotion stability + regressions)

Minimum sample sizes

(If you ignore this, you’ll “promote” on 10 lucky calls.)

Low-volume roles: ≥ 50 calls in W24h

Normal roles: ≥ 200 calls in W24h

Apply role: ≥ 300 applies in W24h (because 1–2 failures can hide)

4. Promotion rubric (CANARY → ACTIVE)

Promotion is role-scoped and class-scoped. You can promote an offer for code_patch_generator while keeping it canary-only for code_reasoner.

4.1 Global hard gates (all offers, any role)

To promote for a role you must pass:

Reliability

HardFailRate(W24h) ≤ 0.8%

RateLimitRate(W24h) ≤ 1.0%

BadRequestRate(W24h) ≤ 1.0% (if higher, it’s contract/prompt mismatch)

Stability

TTFT_p95(W1h) not worse than 2.0× TTFT_p95(W24h baseline)

ITL_p95(W1h) not worse than 2.0× ITL_p95(W24h baseline)

Error rates W5m not spiking above 3× W24h

Economics sanity

BudgetShadow ≤ 2.0 (if you’re in red panic mode, don’t promote new stuff)

PlanShadow ≤ 2.0 (if the plan is nearly exhausted, promotion is meaningless)

If any of these fail: stay CANARY.

4.2 Role-specific promotion gates
code_reasoner (highest risk)

Min sample: ≥ 200 calls (W24h)

HardFailRate(W24h) ≤ 0.5%

RateLimitRate(W24h) ≤ 0.5%

SchemaAdherence (tools/json) ≥ 99% if tools/json used

Online outcome proxy: EscalationRate (reasoner rerun due to failure) ≤ 3%

If it’s not this good, it stays canary or becomes fallback-only.

code_patch_generator

Min sample: ≥ 200 calls

JSON validity (SchemaAdherence) ≥ 98%

HardFailRate ≤ 1.0%

“Rewrite bloat” guard: tokensOutMean for role must be ≤ 1.5× role median
(If it writes full files, it’s failing the whole strategy.)

code_apply_patch

Min sample: ≥ 300 applies

ApplySuccess ≥ 97%

Empty/invalid output ≤ 0.5%

ValidatorPass (if validator enabled) ≥ 95%

EscalationRate ≤ 5% for single-file; ≤ 10% for multi-file (early acceptable)

code_scaffold_fast

Min sample: ≥ 200 calls

HardFailRate ≤ 1.0%

TTFT/ITL stability (no cliffs)

SpeedScore_p95 must be within top 40% of eligible offers for that role
(Otherwise “fast” is marketing, not reality.)

5. Demotion rubric (ACTIVE → DEGRADED / SUSPENDED)

Demotion must be faster and more aggressive than promotion.

5.1 Immediate circuit breaker (→ SUSPENDED)

Trigger if any of these happens in W5m with ≥ 20 calls:

HardFailRate(W5m) ≥ 5%

RateLimitRate(W5m) ≥ 10%

Total outage signature: ≥ 5 consecutive failures

SchemaAdherence(W5m) < 90% on schema-critical roles (patch gen / tools / apply)

Action: move offer to SUSPENDED for cooldown (see section 7).

5.2 Soft demotion (→ DEGRADED)

Trigger if in W1h with ≥ 50 calls:

HardFailRate(W1h) ≥ 2%, OR

RateLimitRate(W1h) ≥ 3%, OR

TTFT_p95(W1h) ≥ 2.5× TTFT_p95(W24h), OR

ITL_p95(W1h) ≥ 2.5× ITL_p95(W24h)

Action: move to DEGRADED (still eligible but heavily penalized + traffic capped).

5.3 Apply-specific demotion

Because apply errors are expensive (they trigger escalations):

ApplySuccess(W1h) < 95% → DEGRADED

ApplySuccess(W5m) < 90% (≥20 applies) → SUSPENDED

ValidatorPass drops by ≥ 5 points vs trailing W24h → DEGRADED

6. Traffic ramp schedule (CANARY → ACTIVE safely)

Traffic is per role, not global. That prevents a new offer from hijacking everything.

Standard ramp (for UF, UB, SH, MQ, CU)

Step 0: 0.5% traffic share (minimum 50 calls)

Step 1: 2% (minimum 200 calls + gates pass)

Step 2: 10%

Step 3: 25%

Step 4: 50%

Step 5: 100% (or keep a permanent cap if you want diversity)

Rule: you only advance one step after passing the promotion gates at that step’s sample size.

Conservative ramp (for VF and anything “preview”)

VF is a reliability trap unless you’re strict.

Step 0: 0.1%

Step 1: 0.5%

Step 2: 2% (max)

Hard rule: VF never exceeds 2% unless you flip a manual “experimental override” flag.

Apply ramp (for AP offers)

Apply affects correctness; ramp slower.

Step 0: single-file only, 1%

Step 1: single-file, 5%

Step 2: single-file, 20%

Step 3: multi-file enabled but validator required, 5%

Step 4: multi-file + validator, 20%

Step 5: multi-file + validator, 50% (keep permanent cap if you want redundancy)

7. Cooldowns and re-entry rules
   SUSPENDED cooldown

First suspension: 15 minutes

Second within 24h: 60 minutes

Third within 7d: 24 hours or manual review required

DEGRADED recovery

To move DEGRADED → ACTIVE:

pass the role’s promotion gates over W1h

AND no circuit-break event in last 2 hours

Canary re-entry

If an offer was SUSPENDED, it re-enters as CANARY at the previous ramp step − 1.

Tell-it-like-it-is:
If you don’t ratchet down after a suspension, you’ll oscillate between “works” and “meltdown” all day.

8. Special rules by offer class (this is where people usually screw up)
   8.1 C0 (Copilot 0× included models)

Treat as abundant, but quality drift is real.

Add a Quality Regression Gate:

If online quality drops by ≥ 5 points (absolute) vs trailing W7d baseline for a role, demote from P→A or A→F in the matrix for that role until it recovers.

8.2 CU (Copilot unit-metered)

When UnitQuotaHealth < 0.7, automatically cap CU traffic share for non-critical roles (e.g., patch_gen/scaffold) so you don’t burn the month early.

When UnitQuotaHealth < 0.3, CU should become fallback-only unless the role’s minQuality forces it.

8.3 UB (daily bucket, e.g., Cerebras Code)

When DailyQuotaHealth < 0.7: move to DEGRADED (pacing protection)

When DailyQuotaHealth < 0.3: fallback-only

When tokensRemainingToday = 0: DISABLED until reset

Also: if RateLimitRate spikes, suspend quickly; daily-bucket services often 429 hard under bursts.

8.4 UF (ultra-fast token-metered)

Promote based on measured TTFT/ITL, not vendor tok/s.

If speed advantage disappears (SpeedScore no longer top-tier), it should stop being preferred for speed-first roles.

8.5 SH (self-host compute-metered)

Add a capacity gate:

If queue_p95 rises sharply OR timeouts spike, it’s usually capacity/batching misconfig.

Demote to DEGRADED until throughput recovers.

8.6 AP (apply specialists)

Strict contract adherence is non-negotiable.

If SchemaAdherence < 98% for apply role in W1h: DEGRADED immediately

If ApplySuccess < 95%: DEGRADED

If ApplySuccess < 90% in W5m: SUSPENDED

8.7 VF (volatile free serverless)

Treat as permanent canary unless it proves itself over time:

Require W7d success stability before even considering ACTIVE for low-stakes roles.

Hard cap traffic share at 2% without explicit override.

9. Auto-rollback triggers (system-wide)

These are not “offer-level”; they change system behavior immediately.

Trigger A: Budget emergency

BudgetShadow ≥ 2.5 (red) OR budgetRemainingUsd < 10% mid-period
Action: enter RED mode:

tighten minQuality/cost caps for non-critical roles

prefer patch DAG (saves cost)

cap VF and CU usage aggressively

Trigger B: Quality regression

EscalationRate increases by ≥ 2× vs trailing W24h for a role
Action:

temporarily switch role preference order to quality-first (even for speed-first roles) until recovery

demote top suspected offer(s) to DEGRADED

Trigger C: Latency meltdown

Median SpeedScore for a role increases ≥ 2× vs baseline
Action:

reduce reliance on queued providers (queue penalty increase)

shift to faster TTFT/ITL offers even if cost rises (short-term)

10. Manual override policy (because automation isn’t perfect)

You will need explicit admin knobs:

ForceDisable(offerId) — immediate DISABLED

ForceCanary(offerId, maxTraffic%) — override automation

ForceActive(offerId, roleScope) — only allowed with an “acknowledge risk” flag

OverrideVFMaxTraffic(%) — explicit override with expiry time

Every override must be logged with:

who

why

scope (roles)

expiry

Tell-it-like-it-is:
If you don’t require expiry on overrides, “temporary” becomes permanent and the system rots.

11. What this produces in practice

New offers don’t hijack routing just because they’re “free” or “fast”.

Apply models get used where they matter (file edits), and get suspended fast if correctness drops.

Daily buckets don’t get drained early day.

Copilot unit plans don’t get burned on low-value tasks when scarcity appears.

The router stays deterministic and explainable under snapshot versions.

If you want the next artifact after this, the most useful one is a “policy-to-snapshot mapping spec”: exactly which fields in OfferSnapshot/EconomicsSnapshot/TelemetrySnapshot/QualitySnapshot must exist to implement every gate above, so nobody “sort of” implements gating and misses the critical pieces.

Model name Context Limit Price In ($\mTok)	Price Out ($\mTok) Cache In ($\mTok)	Cache Out ($\mTok) Subscription Avail? (bool) What? (Provider, name[]) Price by mo (ordered price[]) Usage Lim? (requests or tokens) speed tps

    											 extend and expand this base column list with all needed features and benchmarks + groupings by benchmark type so new benchmarks can be associated\similarized that away

Thought for 7m 19s

Below is a complete catalog schema (spreadsheet‑friendly) that extends your base columns into what you actually need for offer‑first routing + plan-aware economics + TTFT/ITL speed + benchmark-driven quality with sparse coverage.

I’m going to give this as multiple “tabs” (strongly recommended). You can flatten into one mega table, but benchmarks become sparse/ugly fast. The clean approach is:

Tab A — Offers (the routable unit: model+provider+plan+region)

Tab B — Plans (billing instruments, quotas, resets, pools)

Tab C — Providers (API behavior, regions, reliability meta)

Tab D — Benchmark Registry (defines benchmark types + similarity tags)

Tab E — Benchmark Results (long format) (model/offer × benchmark × score)

Tab F — Role Profiles (routing roles, constraint templates, benchmark weights)

Tab G — Telemetry Aggregates (measured speed/cost/verbosity per offer+role)

This design is what lets you “associate/similarize away” new benchmarks: you add them to the Benchmark Registry, not by inventing new columns everywhere.

Tab A — Offer Catalog (routable unit)

One row = one routable endpoint: (provider + model + plan + region + mode)
You route to offer_id, not model_id.

A1) Identity & lineage

offer_id (string, unique) — canonical routing key

model_id (string) — vendor model name

model_family (string) — e.g., “claude”, “gemini”, “qwen”, “deepseek”

model_variant (string) — “flash”, “pro”, “coder”, “reasoning”, etc.

model_version (string) — semver/date hash if available

provider_id (string) — canonical provider key

provider_display_name (string)

plan_id (string) — links to Plans tab

region (string) — “us-east”, “eu”, etc.

endpoint_type (enum) — openai_compat | anthropic | google | azure | custom

endpoint_url (string)

created_at (date)

last_verified_at (date)

lifecycle_state (enum) — disabled | canary | active | degraded | suspended

traffic_cap_pct (number) — per-offer cap (global); optional

allowed_roles (string[] or CSV) — explicit role allowlist (overrides global policy)

A2) Capabilities & modalities

modalities_in (set) — text | image | audio | video | files

modalities_out (set) — text | image | audio | json

tools_supported (bool)

parallel_tool_calls (bool)

json_mode_supported (bool)

structured_output_supported (bool)

logprobs_supported (bool)

streaming_supported (bool)

reasoning_mode_supported (bool) — if distinct from normal

system_prompt_supported (bool)

function_calling_style (enum) — openai | anthropic | json_schema | none

tooling_notes (text)

A3) Context and output constraints

max_context_tokens (int)

max_output_tokens (int)

max_images_per_request (int)

max_audio_seconds_in (int)

max_file_bytes_in (int)

max_tool_calls (int)

supports_long_context_attention (bool) — if you care

supports_cache (bool) — prompt caching semantics exist

A4) Apply/Patch contract support (specialist)

prompt_contract_type (enum) — none | apply_v1 | other

prompt_contract_required_format (text) — the canonical contract string

apply_mode_supported (bool) — true for Morph/Relace class

apply_expected_output (enum) — full_file_only | diff | json_patch | unknown

A5) Hard provider/runtime limits (rate & quota)

(These are offer-level because they vary by provider/plan/region.)

rpm_limit (number)

tpm_limit (number)

rps_burst_limit (number)

concurrency_limit (number)

tokens_per_day_cap (number) — for daily buckets

prompts_per_window_cap (number)

window_seconds (number)

max_requests_per_user_per_day (number) — if applicable

rate_limit_notes (text)

A6) Pricing primitives (raw list pricing, not “effective”)

Keep these even if you compute blended costs later.

price_in_usd_per_mtok (number)

price_out_usd_per_mtok (number)

cache_read_usd_per_mtok (number)

cache_write_usd_per_mtok (number)

image_price_usd_per_unit (number) — define unit in notes

audio_price_usd_per_minute (number)

minimum_billable_unit (enum) — token | request | second | none

billing_rounding (text) — if known

A7) Subscription / pooled-bucket semantics (offer-side pointers)

Because you often have “one subscription covers multiple models.”

subscription_available (bool)

subscription_pool_id (string) — maps to Plans pooling

subscription_pool_share_notes (text) — “shared across X models, dynamic”

monthly_price_usd_tiers (string) — ordered list like 20|200|... (or keep in Plans)

A8) Copilot/premium-unit semantics (when applicable)

unit_metering_type (enum) — none | premium_units

unit_multiplier (number) — can be 0

is_included_0x (bool)

unit_billing_notes (text)

A9) Self-host compute metadata (when offer is SH)

compute_type (enum) — gpu_rental | owned | managed

gpu_type (string) — H100/H200/4090/B200 etc.

gpu_count (int)

vram_gb (int)

quantization (string) — FP16/BF16/FP8/INT8/INT4

runtime_stack (enum) — nim | vllm | trtllm | tgi | custom

max_batch_size (int)

usd_per_gpu_hour (number)

cost_model_notes (text)

A10) Reliability / risk / volatility

sla_uptime (number) — if any

volatility_score (0–1) — promo/preview/rotating = high

is_preview (bool)

promo_active (bool)

promo_start (date)

promo_end (date)

known_deprecation_date (date)

compliance_tier (enum) — none | internal_ok | pii_ok | regulated_ok

data_retention_policy (enum) — unknown | none | short | long

logging_allowed (bool)

risk_notes (text)

A11) Measured performance (telemetry aggregates pointers)

You can store these here or keep them in Telemetry tab; I prefer Telemetry tab, but include “latest snapshot” columns for convenience.

ttft_p50_ms

ttft_p95_ms

itl_p50_ms

itl_p95_ms

queue_p95_ms

tps_stream_p50

tps_stream_p95

throughput_tokens_per_sec_at_ref_concurrency

ref_concurrency (int)

error_rate_1h

rate_limit_429_rate_1h

schema_adherence_1h

tokens_out_mean_1h

apply_p95_ms_per_file (apply offers)

last_perf_sample_n (int)

A12) Derived indices (optional but useful)

These are computed outputs you may cache for fast routing.

index_speed_role_fastchat

index_speed_role_scaffold

index_cost_effective_usd_per_mtok_blended (your learned blended)

index_quality_coding

index_quality_reasoning

index_quality_math

index_quality_tooluse

index_quality_apply

index_confidence_overall (0–1)

Tab B — Plan Catalog (billing instruments)

One row = one plan/subscription/credit pool. Offers reference plan_id.

B1) Identity

plan_id

provider_id

plan_display_name

plan_type (enum) — payg_token | fixed_bucket | daily_bucket | weighted_units | prompt_rate_limited | volatile_free | compute_metered

currency (e.g., USD)

billing_cycle (enum) — monthly | daily | rolling_window | none

reset_day_of_month (int) or reset_timestamp

timezone (string)

B2) Fixed fees & tiers

monthly_fee_usd

tier_prices_usd (ordered list)

tier_criteria (text)

B3) Entitlements (what you “get”)

included_tokens_per_month_est (number) — if known/assumed

included_tokens_per_day (number) — daily bucket

included_units_per_month (number) — premium units (Copilot)

included_prompts_per_window (number)

window_seconds

included_models_pool (string[]) — models sharing same pool

pool_id (string) — shared bucket group

B4) Overages & unit pricing

overage_in_usd_per_mtok

overage_out_usd_per_mtok

unit_overage_usd (e.g., per premium request unit)

overage_rules_notes

B5) Usage state (keep as separate “Plan State” snapshot in production, but you can track columns)

units_used_mtd

units_remaining

tokens_used_today

tokens_remaining_today

tokens_used_mtd

tokens_remaining_mtd_est

tpm_remaining

rpm_remaining

last_usage_update_at

B6) Shadow pricing state (computed)

budget_shadow

plan_shadow_monthly

plan_shadow_daily

plan_shadow_units

effective_unit_cost_usd_per_mtok_in

effective_unit_cost_usd_per_mtok_out

Tab C — Provider Catalog (integration + behavior)

One row per provider (OpenRouter, Vercel AI Gateway, Cerebras, NVIDIA build, etc.)

provider_id

provider_display_name

provider_type (enum) — aggregator | direct | selfhost | gateway

api_style_supported (set) — openai_compat | anthropic | google | custom

auth_methods (set) — api_key | oauth | jwt | session_cookie | none

regions_supported (string[])

supports_usage_reporting (bool) — do they return tokens?

supports_cache_billing_reporting (bool)

rate_limit_headers_present (bool)

latency_region_affinity (text)

status_page_url (string)

known_incidents_notes (text)

provider_volatility_score (0–1)

default_circuit_breaker_policy (text)

Tab D — Benchmark Registry (THIS is what enables “similarize new benchmarks away”)

One row per benchmark definition. New benchmark = add a row here, not new columns.

D1) Identity

benchmark_id (string, unique)

benchmark_name (string)

benchmark_version (string/date)

publisher (string)

public_url_or_ref (string) — optional

license (string) — optional

D2) Benchmark grouping (the key part)

Use a taxonomy that stays stable even as benchmarks change.

benchmark_type_group (enum) — see list below

benchmark_subtype (string) — optional finer bucket

primary_domain (enum) — coding | math | science | general | multilingual | safety | long_context | tool_use | editing

modalities_required (set) — text | image | audio | tools | retrieval

task_format (enum) — mcq | freeform | code_exec | agentic | retrieval | diff_apply | json_structured

scoring_direction (enum) — higher_better | lower_better

score_unit (enum) — percent | pass_rate | score | elo | exact_match | f1 | unknown

difficulty_band (enum) — easy | medium | hard | mixed | unknown

D3) Similarity tags (for “associate/similarize away”)

These tags let you map new benchmarks to existing weighting logic.

skill_tags (string[]) — e.g., debugging, planning, refactoring, tool_use, formal_math, code_gen, long_context_recall

contamination_risk (enum) — low | medium | high | unknown

eval_harness_type (enum) — standard | vendor_reported | third_party | internal

recommended_weight_default (number) — starting weight within its group

normalization_method (enum) — zscore_within_benchmark | minmax | logistic | none

notes (text)

D4) Benchmark type groups (recommended stable set)

Use these as benchmark_type_group:

coding_synthesis (write code from scratch)

coding_debugging (bugfix, reasoning)

coding_repo_swe (repo-level engineering, SWE-style)

coding_editing_apply (diff/apply correctness, file editing)

math_competition (GSM/MATH-like)

science_qa (hard science Q/A)

general_reasoning_knowledge

instruction_following_format (JSON/constraints)

tool_use_agents (function calling, agent loops)

long_context_retrieval (needle-in-haystack, doc recall)

multilingual

safety_alignment

truthfulness_hallucination

multimodal_vision (if you need it)

This list is the “anchor.” New benchmarks get slotted into one (or multiple) groups via tags.

Tab E — Benchmark Results (long format)

One row per (offer or model) × benchmark × score × eval context.

Core columns:

subject_type (enum) — offer | model_family | model
(I recommend offer when provider matters; model when it doesn’t.)

subject_id (offer_id or model_id)

benchmark_id (links to registry)

score_value (number)

score_unit (optional override)

evaluation_date (date)

source_type (enum) — vendor_release | third_party | internal_manual | internal_eval

source_ref (text/url)

confidence (0–1) — how much you trust this number

coverage_note (text) — if partial eval / subset

eval_settings_hash (string) — to detect incompatible setups

prompting_style (enum) — zero_shot | few_shot | tool_augmented | unknown

temperature (number) — if known

pass_at_k (int) — if relevant

notes (text)

Why long format matters: you’ll have missing scores and new benchmarks constantly. Wide columns are pain.

Tab F — Role Profiles (routing roles + benchmark weights)

One row per role.

F1) Identity

role_id (string) — e.g., code_reasoner

role_description (text)

role_class (enum) — reasoning | generation | editing | apply | agentic

F2) Hard constraints template

requires_tools (bool)

requires_json_mode (bool)

requires_prompt_contract_type (enum) — none | apply_v1

min_context_tokens_default (int)

min_quality_default (0–1)

max_cost_usd_default (number)

max_speed_score_default (number)

F3) Optimization order

lexi_order (string) — e.g., quality_desc,cost_asc,speed_asc

F4) Benchmark weighting by type group (this is the “similarization” hook)

Columns (numbers that sum to 1.0 or 100):

w_coding_synthesis

w_coding_debugging

w_coding_repo_swe

w_coding_editing_apply

w_math_competition

w_science_qa

w_general_reasoning_knowledge

w_instruction_following_format

w_tool_use_agents

w_long_context_retrieval

w_multilingual

w_safety_alignment

w_truthfulness_hallucination

w_multimodal_vision

F5) Online quality signals (weights)

w_tests_pass_rate

w_lint_pass_rate

w_escalation_rate (negative weight)

w_schema_adherence

w_apply_success (for apply)

w_validator_pass (for apply/multi-file)

online_weight_ramp_k (int) — how fast online overrides offline

Tab G — Telemetry Aggregates (measured reality)

One row per (offer_id × role_id × window).
This is where you learn speed, conciseness, real cost.

G1) Keys

offer_id

role_id

window (enum) — 5m | 1h | 24h | 7d

as_of (date-time)

sample_n (int)

G2) Speed profile

queue_p50_ms, queue_p95_ms

ttft_p50_ms, ttft_p95_ms

itl_p50_ms, itl_p95_ms

tps_stream_p50, tps_stream_p95

speed_score_p50, speed_score_p95 (your composite)

apply_p95_ms_per_file (apply role)

G3) Reliability

success_rate

error_rate_total

error_rate_429

error_rate_5xx

error_rate_timeout

error_rate_schema

consecutive_fail_max

G4) Conciseness / verbosity (this directly affects “speed” in practice)

tokens_in_mean

tokens_out_mean

tokens_out_p95

verbosity_ratio_mean (out / in)

turns_per_session_mean (if you track)

retry_rate (how often you had to rerun)

escalation_rate (for DAGs)

G5) Cost reality (optional but extremely useful)

Even if providers don’t bill tokens consistently, you can learn effective cost.

effective_usd_per_1k_out_tokens (learned)

effective_usd_per_mtok_total (learned blended)

cache_hit_rate (if caching exists)

avg_request_cost_usd (estimated)

Benchmark grouping “similarization” rule (how new benchmarks plug in)

When a new benchmark appears, you do not add new routing logic.

You:

Add a row to Benchmark Registry with:

benchmark_type_group

skill_tags

task_format

normalization_method

Add whatever scores you have in Benchmark Results.

Your quality index for a role uses RoleProfile weights by benchmark_type_group, so the new benchmark automatically contributes to the correct bucket.

If you want more nuance: allow a benchmark to map to multiple groups with fractional membership:

add group_membership as JSON like {"coding_repo_swe":0.7,"tool_use_agents":0.3}

If you insist on “single wide table” columns

You can keep Offers as the wide table and add only bucketed benchmark aggregates as columns:

bench_coding_synthesis_score

bench_coding_debugging_score

bench_coding_repo_swe_score

bench_coding_editing_apply_score

bench_math_score

… etc for each benchmark_type_group
…and still store raw benchmark values in Tab E long format.

That’s the only wide approach that won’t become unmaintainable.

If you want the next step: I can propose a canonical naming convention for offer_id, model_id, benchmark_id, plus an elicitation form layout (what you fill in manually per provider/model) so you don’t end up with inconsistent strings that break joins.

I need a psreadsheet for mvp \ personal data analysis.

Also need a provider, harness table and apretos for each indiv, pair, trio, and role\task types
Thought for 40m 51s
Answer now

Built you an MVP workbook with the tables + Pareto sheets you asked for (individual offers, pairs, trios; all selectable by role/task).

helios_router_mvp_personal_analysis.xlsx

What’s inside (tabs)

CONTROL: monthly budget pacing ($600 default), “day of month”, telemetry window

PROVIDERS: provider catalog

HARNESSes: harness/client catalog (Codex CLI, Roo/Kilo/Opencode, etc.)

PLANS: subscriptions/quotas + shadows (monthly, daily, unit-based)

OFFERS: offer-first catalog (provider+model+plan+region)

ROLES: role/task types with benchmark-group weights + speed mode

BENCHMARKS / BENCH_RESULTS: benchmark registry + long-format sparse results (for later quality indexing)

TELEMETRY: TTFT/ITL/TPS/etc per offer×role×window

INDICES: per offer×role Cost/Speed/Quality fields used for Pareto

PARETO_1: Pareto frontier for individual offers for a selected role

COMBOS_CONFIG: pick up to 20 candidate offers + set aggregation rules

PARETO_PAIRS / PARETO_TRIOS: Pareto frontier for pairs/trios as routing sets for the selected role

DASHBOARD: quick counts + Pareto stats

How to use (fast)

In CONTROL, set day-of-month + spend-to-date (budget shadow will adjust automatically).

Fill/update PLANS, OFFERS, and at least a few rows in TELEMETRY.

In INDICES, fill quality_score + quality_conf (until you wire benchmarks).

In PARETO_1, pick a role_id (cell B4) and inspect on_pareto_front.

In COMBOS_CONFIG, paste 10–20 offer_ids (ideally those on the Pareto front). Then check PARETO_PAIRS and PARETO_TRIOS.

If you want, next I can add a “Quality rollup” sheet that turns BENCH_RESULTS into role-quality automatically (sparse coverage + confidence penalties) without adding any new columns when new benchmarks appear.

Simple 1pg

Here’s the simple 1-page MVP spreadsheet:

👉
helios_router_1pg_simple_mvp.xlsx

What’s inside (single sheet)

Columns:

Offer ID

Provider

Model

Role/Task

Quality (0–1)

Speed Score (lower = better)

Effective Cost ($ per call)

On Pareto? (manual yes/no)

Notes

That’s it. Clean and minimal.

You just:

Fill Quality, Speed, Cost.

Manually mark Pareto (or sort by cost/speed/quality to eyeball frontier).

Duplicate rows to test pairs/trios if needed.

If you want, I can now make a 1-page version that auto-calculates Pareto with formulas only (no macros).

Streamlit app?

Yes. Here’s a Streamlit MVP that matches your 1-page sheet idea but actually computes:

per-role filtering

Pareto frontier for individual offers

optional pairs/trios Pareto (limited N so it stays fast)

sortable table + download CSV

1. Install
   python -m venv .venv
   source .venv/bin/activate # Windows: .venv\Scripts\activate
   pip install streamlit pandas numpy openpyxl
2. Save this as app.py
   import itertools
   import math
   from dataclasses import dataclass
   from typing import List, Tuple, Optional, Dict

import numpy as np
import pandas as pd
import streamlit as st

st.set_page_config(page_title="Helios Router MVP", layout="wide")

# -----------------------------

# Helpers

# -----------------------------

def pareto_front(df: pd.DataFrame,
minimize_cols: List[str],
maximize_cols: List[str]) -> pd.Series:
"""
Returns a boolean Series marking non-dominated rows.
Dominance: <= on all minimize, >= on all maximize, and strictly better on at least one.
O(N^2) but OK for small N (MVP).
"""
arr_min = df[minimize_cols].to_numpy(dtype=float)
arr_max = df[maximize_cols].to_numpy(dtype=float) if maximize_cols else np.zeros((len(df), 0))
n =
For the full.

Below is a full Streamlit app (single-file) that supports:

Upload your full MVP workbook (or CSVs)

Manage / view Providers, Harnesses, Plans, Offers, Roles, Benchmarks, Results, Telemetry

Compute effective cost, speed score, quality score (offline benchmarks + online telemetry blend)

Compute Pareto frontiers for:

individual offers

pairs

trios

Export computed tables to CSV (and optionally XLSX)

Save as app.py.

import itertools
import math
from dataclasses import dataclass
from typing import Dict, List, Tuple, Optional

import numpy as np
import pandas as pd
import streamlit as st

# ----------------------------

# App config

# ----------------------------

st.set_page_config(page_title="Helios Router - Full Personal Analysis", layout="wide")

# ----------------------------

# Defaults / constants

# ----------------------------

DEFAULT_ROLE_WEIGHTS = { # benchmark_type_group -> weight
"coding_synthesis": 0.20,
"coding_debugging": 0.20,
"coding_repo_swe": 0.20,
"coding_editing_apply": 0.15,
"tool_use_agents": 0.15,
"instruction_following_format": 0.10,
}

BENCH_GROUPS = [
"coding_synthesis",
"coding_debugging",
"coding_repo_swe",
"coding_editing_apply",
"tool_use_agents",
"instruction_following_format",
"long_context_retrieval",
"math_competition",
"general_reasoning_knowledge",
"multilingual",
"safety_alignment",
"truthfulness_hallucination",
"multimodal_vision",
]

# ----------------------------

# Utility

# ----------------------------

def _safe_float(x, default=np.nan) -> float:
try:
if x is None:
return default
if isinstance(x, str) and x.strip() == "":
return default
return float(x)
except Exception:
return default

def _clamp(x: float, lo: float, hi: float) -> float:
return max(lo, min(hi, x))

def _normalize_weights(w: Dict[str, float]) -> Dict[str, float]:
s = sum(max(0.0, float(v)) for v in w.values())
if s <= 0:
return {k: 0.0 for k in w.keys()}
return {k: max(0.0, float(v)) / s for k, v in w.items()}

def pareto_front_mask(df: pd.DataFrame, minimize: List[str], maximize: List[str]) -> pd.Series:
"""
O(N^2) Pareto mask. True = non-dominated.
Dominance: a dominates b if: - for all minimize cols: a <= b - for all maximize cols: a >= b - and at least one strict inequality.
"""
if df.empty:
return pd.Series([], dtype=bool)

    mins = df[minimize].to_numpy(dtype=float) if minimize else np.zeros((len(df), 0))
    maxs = df[maximize].to_numpy(dtype=float) if maximize else np.zeros((len(df), 0))

    n = len(df)
    keep = np.ones(n, dtype=bool)

    for i in range(n):
        if not keep[i]:
            continue
        for j in range(n):
            if i == j or not keep[i]:
                continue
            # j dominates i?
            no_worse_min = np.all(mins[j] <= mins[i]) if mins.shape[1] else True
            no_worse_max = np.all(maxs[j] >= maxs[i]) if maxs.shape[1] else True
            if not (no_worse_min and no_worse_max):
                continue
            strict_better = False
            if mins.shape[1]:
                strict_better = strict_better or np.any(mins[j] < mins[i])
            if maxs.shape[1]:
                strict_better = strict_better or np.any(maxs[j] > maxs[i])
            if strict_better:
                keep[i] = False
    return pd.Series(keep, index=df.index)

def choose_default_sheet(dfs: Dict[str, pd.DataFrame], candidates: List[str]) -> Optional[str]:
for c in candidates:
if c in dfs:
return c # fallback: any sheet
return next(iter(dfs.keys()), None)

# ----------------------------

# Data loading

# ----------------------------

EXPECTED_SHEETS = { # If you uploaded the earlier “full MVP” workbook, these are likely present.
"PROVIDERS": ["PROVIDERS", "providers"],
"HARNESSes": ["HARNESSes", "HARNESS", "harnesses"],
"PLANS": ["PLANS", "plans"],
"OFFERS": ["OFFERS", "offers"],
"ROLES": ["ROLES", "roles"],
"BENCHMARKS": ["BENCHMARKS", "benchmark_registry", "benchmarks"],
"BENCH_RESULTS": ["BENCH_RESULTS", "benchmark_results", "bench_results"],
"TELEMETRY": ["TELEMETRY", "telemetry"],
}

def load_workbook(file) -> Dict[str, pd.DataFrame]:
xls = pd.ExcelFile(file)
dfs = {}
for name in xls.sheet_names:
try:
df = pd.read_excel(xls, sheet_name=name) # Normalize columns: strip
df.columns = [str(c).strip() for c in df.columns]
dfs[name.strip()] = df
except Exception:
continue
return dfs

def load_csv(file) -> pd.DataFrame:
df = pd.read_csv(file)
df.columns = [str(c).strip() for c in df.columns]
return df

# ----------------------------

# Core computations

# ----------------------------

@dataclass
class Controls:
budget_monthly_usd: float
spend_to_date_usd: float
day_of_month: int
days_in_month: int
budget_shadow_eps: float # Copilot
copilot_unit_overage_usd: float # Online quality blend
online_k: float # ramp parameter

def compute_budget_shadow(ctrl: Controls) -> float:
remaining = max(ctrl.budget_monthly_usd - ctrl.spend_to_date_usd, 0.0)
frac_elapsed = _clamp(ctrl.day_of_month / max(ctrl.days_in_month, 1), 0.0, 1.0)
expected_remaining = max(ctrl.budget_monthly_usd * (1 - frac_elapsed), 0.0)
ratio = remaining / max(expected_remaining, ctrl.budget_shadow_eps)
return 1.0 / max(ratio, ctrl.budget_shadow_eps)

def plan_shadow_from_remaining(actual_remaining: float, expected_remaining: float, eps: float) -> float:
ratio = actual_remaining / max(expected_remaining, eps)
return 1.0 / max(ratio, eps)

def compute_effective_cost_per_call(
offer_row: pd.Series,
plan_row: Optional[pd.Series],
ctrl: Controls,
tok_in: float,
tok_out: float,
cache_hit_tokens: float = 0.0
) -> float:
"""
Supports: - payg_token pricing from offers columns: price_in/out, cache read/write - fixed bucket: monthly_fee / tokens_covered_est (if present) - daily bucket: daily cap + tokensUsedToday (if present) -> daily shadow + baseline EUC - copilot weighted units: overage * multiplier * unit shadow; 0x => small floor - volatile_free: floor + volatility penalty
"""
budget_shadow = compute_budget_shadow(ctrl)

    plan_type = None
    if plan_row is not None and "plan_type" in plan_row:
        plan_type = str(plan_row["plan_type"]).strip().lower()

    # Offer fields (fallbacks)
    price_in = _safe_float(offer_row.get("Price In ($\\mTok)", offer_row.get("price_in_usd_per_mtok", np.nan)))
    price_out = _safe_float(offer_row.get("Price Out ($\\mTok)", offer_row.get("price_out_usd_per_mtok", np.nan)))
    cache_in = _safe_float(offer_row.get("Cache In ($\\mTok)", offer_row.get("cache_read_usd_per_mtok", np.nan)))
    cache_out = _safe_float(offer_row.get("Cache Out ($\\mTok)", offer_row.get("cache_write_usd_per_mtok", np.nan)))
    volatility = _safe_float(offer_row.get("volatility_score", offer_row.get("Volatility", 0.2)), 0.2)
    unit_multiplier = _safe_float(offer_row.get("unit_multiplier", offer_row.get("unitMultiplier", np.nan)), np.nan)
    is_0x = str(offer_row.get("is_included_0x", offer_row.get("is_0x", ""))).strip().lower() in ("true", "1", "yes", "y")

    # Token math helper
    def token_cost(mt_in, mt_out, p_in, p_out):
        return (mt_in / 1_000_000.0) * p_in + (mt_out / 1_000_000.0) * p_out

    # Default: if no plan info, treat as payg token pricing
    if plan_type is None or plan_type in ("payg_token", "payg", ""):
        # Apply cache hit reduction (cache read is cheaper). For MVP:
        # - treat cache_hit_tokens as "read" and remaining as normal input.
        hit = max(cache_hit_tokens, 0.0)
        norm_in = max(tok_in - hit, 0.0)
        base = token_cost(norm_in, tok_out, price_in if not np.isnan(price_in) else 0.0, price_out if not np.isnan(price_out) else 0.0)
        if not np.isnan(cache_in):
            base += token_cost(hit, 0.0, cache_in, 0.0)
        return base * budget_shadow + (0.002 * volatility)

    if plan_type == "fixed_bucket":
        fee = _safe_float(plan_row.get("monthly_fee_usd", plan_row.get("monthlyFeeUsd", 0.0)), 0.0)
        covered = _safe_float(plan_row.get("tokens_covered_est", plan_row.get("included_tokens_per_month_est", np.nan)), np.nan)
        if np.isnan(covered) or covered <= 0:
            # fallback: treat as payg
            base = token_cost(tok_in, tok_out, price_in if not np.isnan(price_in) else 0.0, price_out if not np.isnan(price_out) else 0.0)
            return base * budget_shadow + (0.002 * volatility)
        euc = fee / covered  # $ per token
        base = euc * (tok_in + tok_out)
        # plan shadow optional
        shadow = _safe_float(plan_row.get("plan_shadow", 1.0), 1.0)
        return base * shadow * budget_shadow + (0.002 * volatility)

    if plan_type == "daily_bucket":
        fee = _safe_float(plan_row.get("monthly_fee_usd", 0.0), 0.0)
        cap = _safe_float(plan_row.get("tokens_per_day_cap", plan_row.get("tokensPerDayCap", np.nan)), np.nan)
        used_today = _safe_float(plan_row.get("tokens_used_today", plan_row.get("tokensUsedToday", 0.0)), 0.0)
        if np.isnan(cap) or cap <= 0:
            # fallback
            base = token_cost(tok_in, tok_out, price_in if not np.isnan(price_in) else 0.0, price_out if not np.isnan(price_out) else 0.0)
            return base * budget_shadow + (0.002 * volatility)

        remaining = max(cap - used_today, 0.0)
        frac_elapsed_day = 0.5  # MVP: you can wire real time; set 0.5 midday
        expected_remaining = cap * (1 - frac_elapsed_day)
        eps = 0.12
        day_shadow = plan_shadow_from_remaining(remaining, expected_remaining, eps)

        # baseline EUC: fee spread across (cap * days_in_month)
        euc = fee / max(cap * ctrl.days_in_month, 1.0)
        base = euc * (tok_in + tok_out)
        return base * day_shadow * budget_shadow + (0.002 * volatility)

    if plan_type == "weighted_units":
        # Copilot-like. If 0x, treat as near-zero floor with volatility.
        if is_0x or (not np.isnan(unit_multiplier) and unit_multiplier == 0.0):
            return (0.002 + 0.002 * volatility) * (1.0 + 0.0 * budget_shadow)

        m = unit_multiplier
        if np.isnan(m) or m <= 0:
            m = 1.0

        unit_overage = _safe_float(plan_row.get("unit_overage_usd", ctrl.copilot_unit_overage_usd), ctrl.copilot_unit_overage_usd)

        units_total = _safe_float(plan_row.get("units_included_per_month", plan_row.get("unitsIncludedPerMonth", np.nan)), np.nan)
        units_used = _safe_float(plan_row.get("units_used_mtd", plan_row.get("unitsUsedMTD", 0.0)), 0.0)
        if np.isnan(units_total) or units_total <= 0:
            # no idea -> treat as payg per-request
            return (unit_overage * m) * budget_shadow + (0.002 * volatility)

        units_remaining = max(units_total - units_used, 0.0)
        frac_elapsed = _clamp(ctrl.day_of_month / max(ctrl.days_in_month, 1), 0.0, 1.0)
        expected_remaining = units_total * (1 - frac_elapsed)
        unit_shadow = plan_shadow_from_remaining(units_remaining, expected_remaining, 0.12)

        implied = unit_overage * m
        return implied * unit_shadow * budget_shadow + (0.002 * volatility)

    if plan_type == "prompt_rate_limited":
        # Convert scarcity into cost floor scaled by throttle risk
        throttle = _safe_float(plan_row.get("throttle_penalty", 0.0), 0.0)
        return (0.004 * (1 + throttle) + 0.002 * volatility) * budget_shadow

    if plan_type == "volatile_free":
        # “Free” but risky: keep it low but not zero.
        throttle = _safe_float(plan_row.get("throttle_penalty", 0.5), 0.5)
        return (0.003 + 0.004 * throttle + 0.004 * volatility) * (1.0 + 0.25 * budget_shadow)

    if plan_type == "compute_metered":
        # MVP: $/call based on effective $/token if you provide it, otherwise small floor.
        usd_per_mtok = _safe_float(plan_row.get("effective_usd_per_mtok", np.nan), np.nan)
        if np.isnan(usd_per_mtok) or usd_per_mtok <= 0:
            return (0.004 + 0.002 * volatility) * budget_shadow
        return ((tok_in + tok_out) / 1_000_000.0) * usd_per_mtok * budget_shadow

    # fallback
    base = token_cost(tok_in, tok_out, price_in if not np.isnan(price_in) else 0.0, price_out if not np.isnan(price_out) else 0.0)
    return base * budget_shadow + (0.002 * volatility)

def compute_speed_score(
tel_row: Optional[pd.Series],
role_mode: str,
tok_out_est: float
) -> float:
"""
role_mode: - interactive: queue + ttft + out*itl - bulk: queue + ttft + out/tps - apply: apply_ms_per_file if present else interactive fallback
""" # conservative defaults
queue = 200.0
ttft = 1200.0
itl = 40.0
tps = 50.0
apply_ms = np.nan

    if tel_row is not None:
        queue = _safe_float(tel_row.get("queue_p95_ms", tel_row.get("queueP95Ms", queue)), queue)
        ttft = _safe_float(tel_row.get("ttft_p95_ms", tel_row.get("ttftP95Ms", ttft)), ttft)
        itl = _safe_float(tel_row.get("itl_p95_ms", tel_row.get("itlP95Ms", itl)), itl)
        tps = _safe_float(tel_row.get("tps_stream_p50", tel_row.get("tpsStreamP50", tps)), tps)
        apply_ms = _safe_float(tel_row.get("apply_p95_ms_per_file", tel_row.get("applyP95MsPerFile", np.nan)), np.nan)

    out = max(tok_out_est, 1.0)

    role_mode = (role_mode or "bulk").strip().lower()
    if role_mode == "apply":
        if not np.isnan(apply_ms) and apply_ms > 0:
            return apply_ms
        return queue + ttft + out * itl

    if role_mode == "interactive":
        return queue + ttft + out * itl

    # bulk default
    return queue + ttft + out / max(tps, 1.0)

def build_offline_quality_scores(
offers: pd.DataFrame,
bench_registry: pd.DataFrame,
bench_results: pd.DataFrame,
role_weights: Dict[str, float],
subject_type: str = "offer"
) -> pd.DataFrame:
"""
Returns DataFrame with columns:
subject_id, offline_quality_raw (z-summed), coverage (0..1), offline_quality (0..1)
Strategy: - z-score per benchmark across available subjects - map each benchmark to benchmark_type_group - per subject, sum weights(group) * mean(z in group) - impute missing groups by (global mean z for group = 0) -> effectively 0 contribution - coverage penalty: offline_adj = offline * (0.6 + 0.4*coverage) - squash with sigmoid to 0..1
"""
if offers is None or offers.empty:
return pd.DataFrame(columns=["subject_id", "offline_quality_raw", "coverage", "offline_quality"])

    # Normalize registry columns
    reg = bench_registry.copy() if bench_registry is not None else pd.DataFrame()
    if not reg.empty:
        reg.columns = [c.strip() for c in reg.columns]
    res = bench_results.copy() if bench_results is not None else pd.DataFrame()
    if not res.empty:
        res.columns = [c.strip() for c in res.columns]

    if reg.empty or res.empty:
        # No benchmark data; return NaNs (router will rely on manual/online)
        out = pd.DataFrame({"subject_id": offers.get("offer_id", offers.get("Offer ID", pd.Series(dtype=str)))})
        out["offline_quality_raw"] = np.nan
        out["coverage"] = 0.0
        out["offline_quality"] = np.nan
        return out

    # Required columns in registry/results
    # Registry: benchmark_id, benchmark_type_group
    # Results: subject_type, subject_id, benchmark_id, score_value
    # Try to auto-detect common names
    def colpick(df, options):
        for o in options:
            if o in df.columns:
                return o
        return None

    reg_bench_id = colpick(reg, ["benchmark_id", "Benchmark ID", "bench_id"])
    reg_group = colpick(reg, ["benchmark_type_group", "benchmark_group", "Group", "type_group"])
    res_subject_type = colpick(res, ["subject_type", "Subject Type"])
    res_subject_id = colpick(res, ["subject_id", "Subject ID", "offer_id", "Offer ID", "model_id", "Model ID"])
    res_bench_id = colpick(res, ["benchmark_id", "Benchmark ID", "bench_id"])
    res_score = colpick(res, ["score_value", "Score", "score"])

    if not (reg_bench_id and reg_group and res_subject_id and res_bench_id and res_score):
        out = pd.DataFrame({"subject_id": offers.get("offer_id", offers.get("Offer ID", pd.Series(dtype=str)))})
        out["offline_quality_raw"] = np.nan
        out["coverage"] = 0.0
        out["offline_quality"] = np.nan
        return out

    # Filter to desired subject_type if provided in data
    if res_subject_type and subject_type:
        res_f = res[res[res_subject_type].astype(str).str.lower() == subject_type.lower()].copy()
    else:
        res_f = res.copy()

    # Join to registry to get group
    reg2 = reg[[reg_bench_id, reg_group]].copy()
    reg2.columns = ["benchmark_id", "group"]
    res2 = res_f[[res_subject_id, res_bench_id, res_score]].copy()
    res2.columns = ["subject_id", "benchmark_id", "score_value"]
    res2["score_value"] = res2["score_value"].apply(_safe_float)

    merged = res2.merge(reg2, on="benchmark_id", how="left")
    merged = merged.dropna(subset=["score_value", "group"])

    if merged.empty:
        out = pd.DataFrame({"subject_id": offers.get("offer_id", offers.get("Offer ID", pd.Series(dtype=str)))})
        out["offline_quality_raw"] = np.nan
        out["coverage"] = 0.0
        out["offline_quality"] = np.nan
        return out

    # z-score within each benchmark_id across subjects
    merged["z"] = merged.groupby("benchmark_id")["score_value"].transform(
        lambda s: (s - s.mean()) / (s.std(ddof=0) if s.std(ddof=0) and s.std(ddof=0) > 0 else 1.0)
    )

    w = _normalize_weights({g: role_weights.get(g, 0.0) for g in BENCH_GROUPS})

    # compute mean z per subject per group
    subj_group = merged.groupby(["subject_id", "group"])["z"].mean().reset_index()

    # Pivot groups wide
    pivot = subj_group.pivot_table(index="subject_id", columns="group", values="z", aggfunc="mean")
    pivot = pivot.reindex(columns=BENCH_GROUPS)

    # Weighted sum (missing => NaN -> treat as 0 contribution)
    zmat = pivot.fillna(0.0).to_numpy()
    wvec = np.array([w[g] for g in BENCH_GROUPS], dtype=float)
    raw = zmat @ wvec

    # coverage = fraction of groups with non-missing original values that have positive weight
    weight_mask = (wvec > 0)
    present = (~pivot.isna()).to_numpy() & weight_mask
    denom = max(weight_mask.sum(), 1)
    coverage = present.sum(axis=1) / denom

    # coverage penalty
    raw_adj = raw * (0.6 + 0.4 * coverage)

    # squash to 0..1
    offline = 1 / (1 + np.exp(-raw_adj))

    out = pd.DataFrame({
        "subject_id": pivot.index.astype(str),
        "offline_quality_raw": raw_adj,
        "coverage": coverage,
        "offline_quality": offline
    }).reset_index(drop=True)

    return out

def blend_quality(offline_q: float, online_q: float, n: float, k: float) -> float:
"""
Blend offline and online quality with ramp rho(n)=n/(n+k)
"""
if np.isnan(offline_q) and np.isnan(online_q):
return np.nan
if np.isnan(offline_q):
return online_q
if np.isnan(online_q):
return offline_q
rho = n / (n + k) if (n + k) > 0 else 0.0
return (1 - rho) * offline_q + rho * online_q

# ----------------------------

# UI

# ----------------------------

st.title("Helios Router — Full Personal Data Analysis (Offers • Plans • Benchmarks • Telemetry • Pareto)")

with st.sidebar:
st.header("Data")
uploaded_xlsx = st.file_uploader("Upload XLSX (full workbook)", type=["xlsx"])
uploaded_csvs = st.file_uploader("Or upload CSVs (multi-select)", type=["csv"], accept_multiple_files=True)

    st.divider()
    st.header("Controls")
    budget_monthly = st.number_input("Monthly variable budget ($)", min_value=0.0, value=600.0, step=10.0)
    spend_to_date = st.number_input("Spend-to-date ($)", min_value=0.0, value=0.0, step=10.0)
    day_of_month = st.number_input("Day of month", min_value=1, max_value=31, value=15, step=1)
    days_in_month = st.number_input("Days in month", min_value=28, max_value=31, value=30, step=1)

    copilot_unit_overage = st.number_input("Copilot unit overage ($/unit)", min_value=0.0, value=0.04, step=0.01)

    online_k = st.number_input("Online quality ramp k (bigger = slower ramp)", min_value=1.0, value=200.0, step=10.0)

    st.divider()
    st.header("Pareto settings")
    pareto_minimize_cost = st.checkbox("Minimize cost", value=True)
    pareto_minimize_speed = st.checkbox("Minimize speed score", value=True)
    pareto_maximize_quality = st.checkbox("Maximize quality", value=True)

    max_combo_n = st.number_input("Max offers for pair/trio search (N)", min_value=5, max_value=80, value=25, step=5)
    allow_pairs = st.checkbox("Compute pairs", value=True)
    allow_trios = st.checkbox("Compute trios", value=False)

# ----------------------------

# Load data

# ----------------------------

dfs: Dict[str, pd.DataFrame] = {}
csv_map: Dict[str, pd.DataFrame] = {}

if uploaded_xlsx is not None:
dfs = load_workbook(uploaded_xlsx)
elif uploaded_csvs:
for f in uploaded_csvs:
name = f.name
csv_map[name] = load_csv(f)

# Map expected sheets

tables: Dict[str, pd.DataFrame] = {}

if dfs:
for key, names in EXPECTED_SHEETS.items():
sheet = choose_default_sheet(dfs, names)
if sheet:
tables[key] = dfs[sheet].copy()
else: # If CSVs, user must name them or we guess # We try to infer by filename keywords
for fname, df in csv_map.items():
low = fname.lower()
if "offer" in low:
tables["OFFERS"] = df
elif "plan" in low:
tables["PLANS"] = df
elif "provider" in low:
tables["PROVIDERS"] = df
elif "role" in low:
tables["ROLES"] = df
elif "bench" in low and "result" in low:
tables["BENCH_RESULTS"] = df
elif "bench" in low:
tables["BENCHMARKS"] = df
elif "tele" in low:
tables["TELEMETRY"] = df
elif "harness" in low:
tables["HARNESSes"] = df

missing = [k for k in ["OFFERS", "PLANS", "ROLES"] if k not in tables]
if missing:
st.warning(f"Missing tables: {missing}. You can still explore what loaded, but Pareto/indices will be limited.")

# Show loaded sheets

with st.expander("Loaded tables", expanded=False):
if dfs:
st.write("Sheets in XLSX:", list(dfs.keys()))
st.write("Mapped tables:", {k: v.shape for k, v in tables.items()})

# ----------------------------

# Normalize key columns

# ----------------------------

offers = tables.get("OFFERS", pd.DataFrame()).copy()
plans = tables.get("PLANS", pd.DataFrame()).copy()
roles = tables.get("ROLES", pd.DataFrame()).copy()
providers = tables.get("PROVIDERS", pd.DataFrame()).copy()
harnesses = tables.get("HARNESSes", pd.DataFrame()).copy()
bench_registry = tables.get("BENCHMARKS", pd.DataFrame()).copy()
bench_results = tables.get("BENCH_RESULTS", pd.DataFrame()).copy()
telemetry = tables.get("TELEMETRY", pd.DataFrame()).copy()

def ensure_col(df: pd.DataFrame, old: List[str], new: str) -> pd.DataFrame:
for o in old:
if o in df.columns and new not in df.columns:
df = df.rename(columns={o: new})
return df

# Offers key columns

offers = ensure_col(offers, ["Offer ID", "offerId"], "offer_id")
offers = ensure_col(offers, ["Provider", "provider"], "provider")
offers = ensure_col(offers, ["Model", "model"], "model_id")
offers = ensure_col(offers, ["Plan ID", "planId", "Plan"], "plan_id")
offers = ensure_col(offers, ["Role/Task", "role"], "default_role")

# Plans

plans = ensure_col(plans, ["Plan ID", "planId"], "plan_id")
plans = ensure_col(plans, ["Plan Type", "type"], "plan_type")
plans = ensure_col(plans, ["Monthly Fee ($)", "monthlyFeeUsd", "monthly_fee_usd"], "monthly_fee_usd")

# Roles

roles = ensure_col(roles, ["Role ID", "roleId", "role"], "role_id")
roles = ensure_col(roles, ["Speed Mode", "speed_mode"], "speed_mode")
roles = ensure_col(roles, ["Min Quality", "min_quality"], "min_quality_default")
roles = ensure_col(roles, ["Max Cost ($)", "max_cost_usd"], "max_cost_usd_default")
roles = ensure_col(roles, ["Lexi Order", "lexi_order"], "lexi_order")

# Telemetry

telemetry = ensure_col(telemetry, ["Offer ID", "offerId"], "offer_id")
telemetry = ensure_col(telemetry, ["Role ID", "roleId", "role"], "role_id")
telemetry = ensure_col(telemetry, ["Window", "window"], "window")

# If plans have missing plan_type, default payg_token

if not plans.empty and "plan_type" not in plans.columns:
plans["plan_type"] = "payg_token"

# ----------------------------

# Control object

# ----------------------------

ctrl = Controls(
budget_monthly_usd=float(budget_monthly),
spend_to_date_usd=float(spend_to_date),
day_of_month=int(day_of_month),
days_in_month=int(days_in_month),
budget_shadow_eps=0.12,
copilot_unit_overage_usd=float(copilot_unit_overage),
online_k=float(online_k),
)

budget_shadow_val = compute_budget_shadow(ctrl)

st.caption(f"Computed budget shadow: **{budget_shadow_val:.3f}** (higher = more budget pressure)")

# ----------------------------

# Select role

# ----------------------------

role_list = []
if not roles.empty and "role_id" in roles.columns:
role_list = sorted([r for r in roles["role_id"].dropna().astype(str).unique()])
if not role_list:
role_list = ["code_reasoner", "code_patch_generator", "code_apply_patch", "code_scaffold_fast", "code_small_transform"]

selected_role = st.selectbox("Select role/task", role_list, index=0)

# Determine role speed mode + defaults

role_row = None
if not roles.empty and "role_id" in roles.columns:
rr = roles[roles["role_id"].astype(str) == str(selected_role)]
if not rr.empty:
role_row = rr.iloc[0]

role_speed_mode = str(role_row.get("speed_mode", "bulk") if role_row is not None else "bulk").strip().lower()
role_min_quality = _safe_float(role_row.get("min_quality_default", 0.0) if role_row is not None else 0.0, 0.0)
role_max_cost = _safe_float(role_row.get("max_cost_usd_default", np.inf) if role_row is not None else np.inf, np.inf)

colA, colB, colC = st.columns(3)
with colA:
min_quality_override = st.slider("Min quality (override)", 0.0, 1.0, float(role_min_quality), 0.01)
with colB:
max_cost_override = st.number_input("Max cost per call ($, override)", min_value=0.0, value=float(role_max_cost if math.isfinite(role_max_cost) else 0.25), step=0.01)
with colC:
speed_mode_override = st.selectbox("Speed mode (override)", ["interactive", "bulk", "apply"], index=["interactive","bulk","apply"].index(role_speed_mode if role_speed_mode in ["interactive","bulk","apply"] else "bulk"))

# Token estimate inputs

st.subheader("Request sizing (for cost/speed estimation)")
c1, c2, c3 = st.columns(3)
with c1:
tok_in_est = st.number_input("Estimated input tokens", min_value=0, value=12000, step=500)
with c2:
tok_out_est = st.number_input("Estimated output tokens", min_value=0, value=2500, step=250)
with c3:
cache_hit_est = st.number_input("Estimated cache-hit tokens (input)", min_value=0, value=0, step=500)

# ----------------------------

# Role weights (benchmark groups)

# ----------------------------

st.subheader("Quality model (benchmarks + online telemetry blend)")
with st.expander("Role benchmark weights (type-group)", expanded=False):
weights = {} # If roles table includes weights, use them; else defaults
for g in BENCH_GROUPS:
default = DEFAULT_ROLE_WEIGHTS.get(g, 0.0)
if role_row is not None and f"w_{g}" in roles.columns:
default = _safe_float(role_row.get(f"w_{g}", default), default)
weights[g] = st.number_input(f"w_{g}", min_value=0.0, value=float(default), step=0.01, key=f"w_{g}")
weights = _normalize_weights(weights)
st.caption(f"Normalized (sum=1.0): {sum(weights.values()):.3f}")

# Compute offline quality from benchmarks (optional)

offline_df = build_offline_quality_scores(
offers=offers,
bench_registry=bench_registry,
bench_results=bench_results,
role_weights=weights,
subject_type="offer"
)

# Online quality proxy from telemetry (optional): schema_adherence + (1-error_rate)

def compute_online_quality(tele_row: Optional[pd.Series]) -> Tuple[float, float]:
"""
Returns (online_quality_0_1, sample_n)
Uses: schema adherence and inverse error rate.
If you store more signals (tests pass, escalation), add them.
"""
if tele_row is None:
return (np.nan, 0.0)
err = _safe_float(tele_row.get("error_rate_total", tele_row.get("errorRate", np.nan)), np.nan)
adh = _safe_float(tele_row.get("schema_adherence", tele_row.get("schemaAdherence", np.nan)), np.nan)
n = _safe_float(tele_row.get("sample_n", tele_row.get("sampleN", 0)), 0.0)

    parts = []
    if not np.isnan(err):
        parts.append(1.0 - _clamp(err, 0.0, 1.0))
    if not np.isnan(adh):
        parts.append(_clamp(adh, 0.0, 1.0))
    if not parts:
        return (np.nan, n)
    return (float(np.mean(parts)), n)

# Choose telemetry window

tele_windows = []
if not telemetry.empty and "window" in telemetry.columns:
tele_windows = sorted([w for w in telemetry["window"].dropna().astype(str).unique()])
if not tele_windows:
tele_windows = ["1h", "24h"]

tele_window = st.selectbox("Telemetry window", tele_windows, index=0)

# ----------------------------

# Build per-offer indices for selected role

# ----------------------------

if offers.empty:
st.error("No OFFERS table loaded. Upload XLSX/CSVs with an OFFERS sheet.")
st.stop()

# Join offers -> plans

plans_idx = plans.set_index("plan_id") if not plans.empty and "plan_id" in plans.columns else None

# Filter eligible offers by roleOnly if present

eligible = offers.copy()

# If offers has roleOnly column (CSV), respect it (comma-separated)

if "roleOnly" in eligible.columns:
def roleonly_ok(x):
if pd.isna(x) or str(x).strip() == "":
return True
parts = [p.strip() for p in str(x).split(",")]
return str(selected_role) in parts
eligible = eligible[eligible["roleOnly"].apply(roleonly_ok)]

# If prompt contract required for apply mode, filter apply_v1 offers

if speed_mode_override == "apply": # Require prompt_contract_type == apply_v1 OR promptContract == apply_v1
if "prompt_contract_type" in eligible.columns:
eligible = eligible[eligible["prompt_contract_type"].astype(str).str.lower().eq("apply_v1")]
elif "promptContract" in eligible.columns:
eligible = eligible[eligible["promptContract"].astype(str).str.lower().str.contains("apply_v1")] # else: can't filter reliably; leave but will likely have no apply telemetry

# Merge offline quality into eligible

if not offline_df.empty and "offer_id" in eligible.columns:
eligible = eligible.merge(offline_df, how="left", left_on="offer_id", right_on="subject_id")
else:
eligible["offline_quality"] = np.nan
eligible["coverage"] = 0.0

# For quick manual quality override columns if present

# Expect: Quality (0-1) and optionally confidence

quality_col = None
for cand in ["Quality (0-1)", "quality", "quality_score"]:
if cand in eligible.columns:
quality_col = cand
break

conf_col = None
for cand in ["quality_conf", "Quality Conf", "confidence"]:
if cand in eligible.columns:
conf_col = cand
break

# Build indices table

rows = []
for _, o in eligible.iterrows():
offer_id = str(o.get("offer_id", "")).strip()
if not offer_id:
continue

    plan_id = str(o.get("plan_id", o.get("Plan ID", ""))).strip()
    plan_row = None
    if plans_idx is not None and plan_id in plans_idx.index:
        plan_row = plans_idx.loc[plan_id]
    # Telemetry row for offer+role+window
    tele_row = None
    if not telemetry.empty and "offer_id" in telemetry.columns:
        tsel = telemetry[
            (telemetry["offer_id"].astype(str) == offer_id) &
            ((telemetry["role_id"].astype(str) == str(selected_role)) if "role_id" in telemetry.columns else True) &
            ((telemetry["window"].astype(str) == str(tele_window)) if "window" in telemetry.columns else True)
        ]
        if not tsel.empty:
            tele_row = tsel.iloc[0]

    # Effective cost per call
    cost = compute_effective_cost_per_call(
        offer_row=o,
        plan_row=plan_row if isinstance(plan_row, pd.Series) else (plan_row.iloc[0] if hasattr(plan_row, "iloc") else None),
        ctrl=ctrl,
        tok_in=float(tok_in_est),
        tok_out=float(tok_out_est),
        cache_hit_tokens=float(cache_hit_est),
    )

    # Speed score
    sp = compute_speed_score(tele_row, speed_mode_override, float(tok_out_est))

    # Quality: prefer manual if provided; else offline+online blend
    manual_q = _safe_float(o.get(quality_col, np.nan), np.nan) if quality_col else np.nan
    manual_conf = _safe_float(o.get(conf_col, 1.0), 1.0) if conf_col else 1.0

    online_q, n = compute_online_quality(tele_row)

    offline_q = _safe_float(o.get("offline_quality", np.nan), np.nan)

    if not np.isnan(manual_q):
        q_final = _clamp(manual_q * manual_conf, 0.0, 1.0)
        q_source = "manual"
    else:
        q_final = blend_quality(offline_q, online_q, n=n, k=ctrl.online_k)
        q_source = "offline+online"

    if np.isnan(q_final):
        q_final = 0.0
        q_source = "missing->0"

    rows.append({
        "offer_id": offer_id,
        "provider": o.get("provider", ""),
        "model_id": o.get("model_id", o.get("Model", "")),
        "plan_id": plan_id,
        "role": selected_role,
        "quality": float(q_final),
        "quality_source": q_source,
        "offline_quality": float(offline_q) if not np.isnan(offline_q) else np.nan,
        "coverage": float(o.get("coverage", 0.0) if not pd.isna(o.get("coverage", np.nan)) else 0.0),
        "online_quality": float(online_q) if not np.isnan(online_q) else np.nan,
        "online_n": float(n),
        "cost_usd": float(cost),
        "speed_score": float(sp),
    })

indices = pd.DataFrame(rows)

# Apply min quality and max cost filtering

indices_filtered = indices.copy()
indices_filtered = indices_filtered[indices_filtered["quality"] >= float(min_quality_override)]
indices_filtered = indices_filtered[indices_filtered["cost_usd"] <= float(max_cost_override)]

st.subheader("Offer indices for selected role")
st.caption("These are the inputs to Pareto: minimize cost, minimize speed_score, maximize quality.")

st.dataframe(indices_filtered.sort_values(["quality", "cost_usd"], ascending=[False, True]), use_container_width=True, height=340)

# ----------------------------

# Pareto for individuals

# ----------------------------

st.subheader("Pareto frontier — individual offers")

min_cols = []
max_cols = []
if pareto_minimize_cost:
min_cols.append("cost_usd")
if pareto_minimize_speed:
min_cols.append("speed_score")
if pareto_maximize_quality:
max_cols.append("quality")

if indices_filtered.empty:
st.info("No offers after applying minQuality/maxCost filters.")
else:
mask = pareto_front_mask(indices_filtered, minimize=min_cols, maximize=max_cols)
pareto_df = indices_filtered.copy()
pareto_df["on_pareto"] = mask.values
st.dataframe(pareto_df.sort_values(["on_pareto", "quality", "cost_usd"], ascending=[False, False, True]), use_container_width=True, height=340)

    st.download_button(
        "Download indices (CSV)",
        data=indices_filtered.to_csv(index=False).encode("utf-8"),
        file_name=f"indices_{selected_role}.csv",
        mime="text/csv",
    )
    st.download_button(
        "Download pareto individuals (CSV)",
        data=pareto_df.to_csv(index=False).encode("utf-8"),
        file_name=f"pareto_individuals_{selected_role}.csv",
        mime="text/csv",
    )

# ----------------------------

# Pairs / trios

# ----------------------------

st.subheader("Pareto — pairs / trios (routing sets)")

st.caption(
"Interpretation: a pair/trio is a _routing set_ you can choose from for this role. "
"We compute combined metrics using conservative aggregations:\n"
"- Combined quality = max(quality among set) (best-available when routing)\n"
"- Combined cost = min(cost among set) (cheapest viable route)\n"
"- Combined speed = min(speed among set) (fastest viable route)\n"
"You can change these rules later to match how you actually route within the set."
)

combo_base = indices_filtered.copy()

# Limit N for combinatorial explosion

combo_base = combo_base.sort_values(["quality", "cost_usd"], ascending=[False, True]).head(int(max_combo_n))

def combo_metrics(rows: List[pd.Series]) -> Dict[str, float]:
q = max(r["quality"] for r in rows)
c = min(r["cost_usd"] for r in rows)
s = min(r["speed_score"] for r in rows)
return {"quality": q, "cost_usd": c, "speed_score": s}

def compute_combos(k: int) -> pd.DataFrame:
combos = []
items = [r for _, r in combo_base.iterrows()]
for comb in itertools.combinations(items, k):
m = combo_metrics(list(comb))
combos.append({
"combo_size": k,
"offer_ids": " | ".join([c["offer_id"] for c in comb]),
"providers": " | ".join([str(c["provider"]) for c in comb]),
"models": " | ".join([str(c["model_id"]) for c in comb]),
**m
})
return pd.DataFrame(combos)

if combo_base.empty or len(combo_base) < 2:
st.info("Need at least 2 eligible offers to compute pairs.")
else:
if allow_pairs:
pairs_df = compute_combos(2)
if not pairs_df.empty:
pairs_mask = pareto_front_mask(pairs_df, minimize=min_cols, maximize=max_cols)
pairs_df["on_pareto"] = pairs_mask.values
st.markdown("### Pairs")
st.dataframe(pairs_df.sort_values(["on_pareto", "quality", "cost_usd"], ascending=[False, False, True]),
use_container_width=True, height=320)
st.download_button(
"Download pareto pairs (CSV)",
data=pairs_df.to_csv(index=False).encode("utf-8"),
file_name=f"pareto_pairs_{selected_role}.csv",
mime="text/csv",
)

    if allow_trios:
        if len(combo_base) < 3:
            st.info("Need at least 3 eligible offers to compute trios.")
        else:
            trios_df = compute_combos(3)
            if not trios_df.empty:
                trios_mask = pareto_front_mask(trios_df, minimize=min_cols, maximize=max_cols)
                trios_df["on_pareto"] = trios_mask.values
                st.markdown("### Trios")
                st.dataframe(trios_df.sort_values(["on_pareto", "quality", "cost_usd"], ascending=[False, False, True]),
                             use_container_width=True, height=320)
                st.download_button(
                    "Download pareto trios (CSV)",
                    data=trios_df.to_csv(index=False).encode("utf-8"),
                    file_name=f"pareto_trios_{selected_role}.csv",
                    mime="text/csv",
                )

# ----------------------------

# Data explorer

# ----------------------------

st.subheader("Data explorer (raw tables)")

tabs = st.tabs(["OFFERS", "PLANS", "ROLES", "TELEMETRY", "BENCHMARKS", "BENCH_RESULTS", "PROVIDERS", "HARNESSes"])
tab_map = {
"OFFERS": offers,
"PLANS": plans,
"ROLES": roles,
"TELEMETRY": telemetry,
"BENCHMARKS": bench_registry,
"BENCH_RESULTS": bench_results,
"PROVIDERS": providers,
"HARNESSes": harnesses
}

for t, name in zip(tabs, tab_map.keys()):
with t:
df = tab_map[name]
if df is None or df.empty:
st.info(f"{name} not loaded.")
else:
st.dataframe(df, use_container_width=True, height=320)

st.caption("MVP note: This app assumes you route to _offers_ and compute combo metrics conservatively. Adjust aggregation rules to match your actual routing strategy.")
Run it
pip install streamlit pandas numpy openpyxl
streamlit run app.py
What you need in your workbook (minimum)

To get full functionality, include at least:

OFFERS with offer_id, provider, model_id, plan_id

PLANS with plan_id, plan_type (e.g., payg_token, weighted_units, daily_bucket, etc.)

ROLES with role_id (optional, otherwise defaults)
Optional but recommended:

TELEMETRY with offer_id, role_id, window, and TTFT/ITL/TPS columns

BENCHMARKS and BENCH_RESULTS for offline quality

If you want, I can adapt the parser to your exact current sheet names/columns (your earlier full workbook) so you don’t rename anything.

Ui for CRUD and mgmt, workbook less w\ internal Ui workbook backed by local csv you create and write to?

Yes — do it as a Streamlit CRUD app backed by local CSVs, and treat Excel as export/import only. That gives you an internal “workbook UI” without fighting Excel semantics.

Here’s the plan (no code) for the UI + data model + file layout + flows. If you want, I can then produce the actual Streamlit file(s), but you asked for the approach first.

1. Local “workbook” = folder of CSVs + schema version

Create a single project directory:

helios_data/
meta.json
providers.csv
harnesses.csv
plans.csv
offers.csv
roles.csv
benchmarks.csv
bench_results.csv
telemetry.csv
indices_cache.csv (generated)
pareto_cache.csv (generated)
combos_cache_pairs.csv (generated)
combos_cache_trios.csv (generated)
audit_log.csv (append-only)
backups/
2026-02-20_120000.zip
meta.json

schema_version

created_at

last_migrated_at

default_budget, days_in_month, etc.

Principle: everything editable lives in CSV; everything computed lives in *_cache.csv.

2. UI modules (tabs) = the “internal workbook”

You’ll have a left nav with 8 modules.

A) Dashboard

budget status (shadow)

quota health (units remaining, daily tokens remaining)

top offers per role (pareto)

alerts (429 spikes, apply failure spike, etc.)

B) Providers (CRUD)

Fields: provider_id, name, type, api_style, auth, regions, volatility baseline, notes
Actions:

add/edit/delete

bulk import from CSV

validate uniqueness + required columns

C) Harnesses (CRUD)

Fields: harness_id, name (Codex CLI / Roo / Kilo / Opencode), auth mode, endpoint style, notes
Actions:

add/edit

map harness→provider capabilities (optional)

D) Plans (CRUD)

Fields: plan_id, plan_type, monthly_fee, reset rule, unit_overage, tokens/day cap, etc.
Actions:

add/edit

“update current usage” panel (units_used, tokens_used_today, spend_to_date)

auto-calc: plan shadows + budget shadow (write into indices_cache)

E) Offers (CRUD)

Fields: offer_id, provider_id, model_id, plan_id, region, capabilities, limits, unit_multiplier, contract_type, volatility
Actions:

add/edit

“clone offer” (same model new provider/region)

bulk paste

role allowlist editor

mark lifecycle_state (active/canary/degraded/suspended)

F) Roles (CRUD)

Fields: role_id, speed_mode, constraints defaults, opt order, benchmark-group weights, online weight ramp
Actions:

add/edit

“preview routing policy matrix” for this role

G) Benchmarks (CRUD + registry)

Fields: benchmark_id, group, tags, score direction, normalization
Actions:

add/edit

“add new benchmark” wizard (assign group + tags so it’s automatically weighted)

H) Benchmark Results (CRUD long-format)

Fields: subject_type, subject_id, benchmark_id, score, confidence, date, source
Actions:

add/edit

bulk import (paste)

validate: benchmark_id exists; subject exists

I) Telemetry (CRUD-lite / Import)

This should mostly be imported from your logs, but allow editing.
Fields: offer_id, role_id, window, ttft_p95, itl_p95, tps, error rates, schema adherence
Actions:

import CSV

append new rows

“compute aggregates” button that writes indices_cache

J) Pareto & Combos (Computed view)

select role

compute:

pareto individuals

pareto pairs (limited N)

pareto trios (limited N)

export to CSV / XLSX

3. CRUD behavior and safety rules

If you don’t enforce these, your dataset will rot.

Primary keys

providers: provider_id

harnesses: harness_id

plans: plan_id

offers: offer_id

roles: role_id

benchmarks: benchmark_id

bench_results: composite key (subject_id, benchmark_id, source_ref or timestamp)

telemetry: composite key (offer_id, role_id, window, as_of)

Foreign key validation (app-level)

offers.plan_id must exist

offers.provider_id must exist

bench_results.benchmark_id must exist

bench_results.subject_id must exist (offer/model depending on subject_type)

telemetry.offer_id must exist

telemetry.role_id must exist

Deletion rules

cannot delete provider if offers reference it (must reassign)

cannot delete plan if offers reference it

soft-delete preferred (flag is_deleted), keep audit log clean

Audit log (append-only)

Every edit writes a row:

ts, user, table, action, key, before_json, after_json

4. Data types and “Excel-like” UX

Streamlit will feel “form-y” unless you do it right.

Best UX pattern:

left: searchable table (DataFrame view)

right: details panel with:

Edit form

Save / Cancel

Clone

Delete (if allowed)

plus: Bulk editor via paste textbox for quick entry

For “workbook feel”, keep:

inline editable grid for small tables (roles, providers)

forms for complex tables (offers, plans)

5. How computations work with CSV backend

You do not want recomputation on every keystroke.

Buttons:

Recompute Indices

builds indices_cache.csv from offers + plans + telemetry + bench results

Compute Pareto

writes pareto_cache.csv for selected role

Compute Pairs/Trios

writes combos_cache_pairs/trios.csv

Caching model:

Any CRUD edit sets a dirty flag in meta.json

Recompute buttons clear dirty flag and update cache timestamps

Dashboard warns “cache stale” when dirty

6. Import/export workflow

Import from XLSX once (optional) → populate CSVs

Use UI thereafter

Export:

“Export workbook as XLSX” (generates tabs from CSVs)

“Export selected role pareto as CSV”

This gives you both: internal UI + Excel compatibility.

7. MVP scope (so you don’t overbuild)

If you want this fast, MVP is:

CRUD: Providers, Plans, Offers, Roles

Light CRUD/import: Telemetry, Benchmarks, Bench Results

Compute: Indices + Pareto Individuals + Pairs + Trios

Export: CSV + XLSX

Audit log

Skip fancy auth and multi-user at first (local use).

8. One decision you need to make

For local CSV storage, pick one:

Single-user local (simple, fast)

Local + Git (every change is a commit; great for history)

Local + SQLite (still local, but safer than CSV for concurrency)

You explicitly asked for CSV-backed, so we’ll do (1) and optionally add “Git commit on save”.

If you say “go”, I’ll generate:

the folder structure

starter CSVs with headers aligned to your schema

the Streamlit UI spec into a clean app skeleton (still no code unless you want it) or I can produce the full app.py that implements this CRUD + compute workflow.

yes, single user local+sqlite
