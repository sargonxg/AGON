# Research prompts — frontier capabilities for AGON

A working set of long-form prompts you can paste into Claude Code research (or Perplexity / Deep Research / your own o1-pro instance). Each prompt is self-contained, names the AGON context, specifies what answer-shape we want back, and includes the trade-offs to evaluate. The goal: **become the Tesla computer vision of human friction — text first, then audio, then video — by leaning on frontier Rust-native tooling that the Python ecosystem cannot match end-to-end.**

Use them as-is or stitch them together. Each one is between 200 and 600 words.

---

## 1. Rust-native frontier extraction stack

> AGON is a Rust binary running on Cloud Run that perceives human-conflict primitives (actors, claims, events, commitments, interests, patterns) from unstructured text. Today we call Vertex AI Gemini 2.5 Flash with a JSON-schema-constrained prompt. The LLM is the bottleneck on latency, cost, and reproducibility. We want a substantially Rust-native extraction pipeline that complements (and where possible replaces) round-trips to a frontier LLM.
>
> Research and produce a comparison table covering Rust crates and bindings that, as of 2026, can plausibly run inside a single Cloud Run container (≤ 250 MB image, ≤ 1 vCPU / 1 GiB RAM, ≤ 4 s cold start) and contribute to extraction of typed primitives from English text. Specifically evaluate:
> - **`candle`** (HuggingFace) vs **`burn`** (Tracel) vs **`tract`** for in-binary neural inference of small classifiers (DistilBERT-sized) — speed, model coverage (safetensors / GGUF / ONNX), GPU/CPU/Metal backends, AOT vs eager.
> - **`fastembed`** vs **`text-embeddings-router`** (HuggingFace TEI bindings) for sentence embeddings.
> - **`mistral.rs`** vs **`llama.cpp`** (via `llama-cpp-rs` / `llm` crates) for fully-local quantized LLM inference (target: Llama 3.x 8B-int4 or Qwen 2.5-7B). Can these realistically run inside 1 GiB on Cloud Run for span-level chores like "is this a commitment?" — what's the latency floor?
> - **`spacy-rs`** vs **`rust-bert`** vs **`pyo3` + spaCy** for classical NLP (NER, POS, dependency parsing).
> - **`stanza-rust`** or **`udpipe-rs`** bindings (do they exist? what's the state in 2026?).
> - **`whisper-rs`** vs **`moonshine-rs`** for speech-to-text — relevant for AGON Phase 3 (audio ingestion).
>
> Score each on: (a) does it actually compile and run on a Debian-12 slim base today, (b) cold-start delta, (c) inference latency on a typical ACO chunk (≤ 2k tokens), (d) maintenance velocity (last 12 months commits), (e) license. Recommend the smallest credible set of crates that, combined, would let AGON pre-filter 80% of LLM round-trips while preserving accuracy on the ACO ontology.

---

## 2. Pre-canonical text understanding

> AGON has a deterministic Rust pass (`aco-server::pretransform`) that runs *before* the LLM call: it detects dialogue turns, normalizes speaker labels, identifies temporal markers, and emits a "pre-canonical envelope" the prompt can reference. Today the pass is a regex on `Speaker:` / `Speaker (ts):` patterns and a small `FormatHint` enum. We want to push it dramatically further.
>
> Research the state of the art in *deterministic Rust-native* document-structure understanding for these specific tasks, and recommend libraries:
> - **Speaker diarization from text** — multi-document threading, alias resolution within a single conversation when speakers don't label themselves (e.g. group chats with quoted replies).
> - **Temporal-marker extraction** — turning "Mon 09:14", "last Thursday", "Q3 2024", "the morning of" into ISO timestamps with confidence intervals. Look at `chrono-english`, `parse_duration`, `hifitime`, `iso8601-timestamp`. Compare against Python's `dateparser` and `Duckling`.
> - **Quote-and-mention extraction** — identifying which token spans are quoted speech, which are paraphrased, which are nested. Existing crates? Should we wrap `pulldown-cmark` for markdown structure, `lol_html` for HTML, `lopdf` for PDF, or write our own state-machine?
> - **Discourse-marker segmentation** — identifying "however", "but", "actually", "wait —", "to be clear", "that's not what I said" as discourse-shift markers that often precede contradictions and DARVO moves. Are there Rust implementations of CCG / RST / SDRT?
> - **Co-reference resolution at the chunk level** — even small classical models like Stanford coref. Is there a Rust implementation? Could `candle` load a fine-tuned BERT-coref model usefully?
>
> Output: a ranked playbook for what to add to `aco-server::pretransform` in the next 30 days, with crate recommendations, expected accuracy lift, and per-call latency budget.

---

## 3. Pattern detection beyond LLM prompting

> AGON currently asks Vertex AI to flag DARVO, gaslighting, stonewalling, contempt, criticism, defensiveness, repair attempts, bids for connection, and triangulation. This is fragile: the LLM hallucinates patterns about half the time on edge cases, and we have no calibration data.
>
> Research the state of the art in **interpersonal-conflict pattern recognition that combines symbolic rules with light neural inference**. Specifically:
> - The Gottman lab's affect-coding ladder (SPAFF, RCISS) — has anyone re-implemented these as deterministic detectors? What text features correlate with each code? Is there a Rust implementation or even a clean Python one we could port?
> - DARVO (Deny / Attack / Reverse Victim & Offender) — Sarah Harsey & Jennifer Freyd's coding scheme. Find the linguistic markers (modal shifts, second-person attribution, accusation reversal). Could we encode these as `ascent` Datalog rules over the ACO graph?
> - Coercive control linguistic markers — Evan Stark, Lisa Aronson Fontes. What span-level features map to control tactics?
> - Gaslighting detection — recent (2024–2026) NLP papers. Any open datasets? Any Rust-runnable classifiers?
> - Repair attempts (Gottman) — bid-and-response detection. Could a small classifier in `candle` flag these with high recall, letting the LLM confirm?
>
> Output: (a) a one-page table of patterns × linguistic markers × algorithmic detectability; (b) a concrete proposal for which patterns should be detected purely in Rust (rule-based or `candle`-classifier) vs which should remain LLM-flagged + Rust-confirmed; (c) source datasets if any.

---

## 4. Computer-vision-of-friction roadmap

> The tagline is "Tesla computer vision of human friction." Tesla's stack is: 8 cameras → per-camera neural perception → fusion into a single 4D world model → planning → control. AGON's stack today is: text → 1 LLM extractor → JSON → render. We want to evolve toward the Tesla shape.
>
> Lay out a 6-quarter roadmap for moving AGON from text-only to text + audio + video + structured records (calendar, IM, email), with the constraint that the runtime stays mostly Rust and stays deployable on a single Cloud Run service (or close approximation).
>
> Address explicitly:
> - **Audio**: `whisper-rs` for STT, then re-run text pipeline; speaker diarization (`pyannote` rewrite in Rust? wrap via `pyo3`? alternative?); affect from prosody (`opensmile` Rust port? do we need it or is text affect sufficient?).
> - **Video**: lip-read confirmation of contested utterances? body-language affect markers? Are there pure-Rust crates for any of this in 2026?
> - **Multimodal fusion**: if a deposition has a recorded video + a transcript that disagree on a single word, how does AGON's fusion layer represent both with provenance? What does the data model look like?
> - **Sensor design**: in the Tesla analogy, "sensors" are typed; what is the parallel for human conflict? Voice tremor, eye contact, response latency, syntactic divergence, modal verb shift, evidential markers (Aikhenvald). Catalog 15+ candidate sensors with their input modality and the Rust crate that would compute them.
>
> Output: 6-quarter milestone plan, 15-sensor catalog, dependency graph between sensors (which require which others).

---

## 5. Sovereign + private deployment

> AGON's customer base will include law firms, family-mediation services, HR investigators, and diplomatic teams. They will not tolerate sending evidence text to Google. We need an in-tenant-VPC or fully-on-prem deployment path.
>
> Research and recommend the minimal Rust-native LLM stack that runs inside a customer's VPC with no Vertex AI dependency, while keeping the same ACO ontology output. Address:
> - What's the strongest open-weight model (≤ 70B parameters, ≤ 16 GiB VRAM for int4) for structured JSON output in 2026? Llama 3.3 70B? Qwen 3? Mistral Large 2? DeepSeek V3? Compare on schema-constraint adherence specifically.
> - Rust runtimes: `mistral.rs` vs `llama.cpp` (via bindings) vs `vllm` (Python-only, exclude) vs `text-generation-inference` Rust client. Which actually serves JSON schemas natively?
> - Quantization formats: GGUF vs AWQ vs SmoothQuant vs MLX in 2026 — what's the trade-off matrix on RTX 4090 / H100 / Mac Studio M4 Ultra?
> - Air-gapped IAP / mTLS / SAML-based auth in front of the Cloud Run alternative — Cloudflare Tunnel? Tailscale Funnel? Vanilla Caddy + mTLS?
> - Audit-log compliance: SOC 2 / HIPAA / GDPR / EU AI Act — what data fields must AGON log, what must it not log, how does the `audit_log` table satisfy each regime?
>
> Output: a deployment matrix — Cloud Run (current) / customer GCP project / customer AWS / on-prem VM / Mac mini in a conference room — with per-row gaps and recommended LLM stack.

---

## 6. Frontier ontology research

> The ACO (Agentic Conflict Ontology) has eight primitives: Actor, Claim, Interest, Constraint, Leverage, Commitment, Event, Narrative. The locked v0.1 spec is good for v0.1. We will need v0.2.
>
> Research what's currently understood (2024–2026) in the formal modelling of:
> - **Stories** as a separate primitive from Narrative — Bruner's "narrative knowing", contemporary work in moral psychology (Haidt, Mercier).
> - **Frames** in the Lakoff / Goffman sense — already implicit in our Narrative primitive, but should it be split out as its own type with explicit framing relations?
> - **Power** as a first-class primitive — currently captured indirectly via Leverage + asymmetry scores. Should AGON have a typed Power node with role (formal / informal / structural / coercive / referent / expert)?
> - **Affect** as a typed primitive — we have AffectMarker as an extension, but it might warrant primitive status given prosodic / video evidence in Phase 3.
> - **Identity** — a claim says "I never agreed", but identity-protecting claims ("I am not the kind of person who breaks commitments") have different rebuttal dynamics.
> - **Norms** vs **Constraints** — current spec collapses them; should norms (descriptive social regularities) be split from constraints (binding rules)?
>
> Output: a v0.2 ontology proposal, additions and refinements only (no removals — v0.1 stays stable), with citations to the underlying literature and a one-paragraph rationale per change.

---

## 7. Real-time streaming + multi-instance dashboard

> AGON's dashboard is single-instance today. Each browser hits one Cloud Run instance. We want a future where: multiple analysts work on the same case, multiple Cloud Run instances autoscale, and a new extraction completed on instance A immediately appears on every dashboard subscribed to that case.
>
> Research and recommend a Rust-native cross-instance event bus that fits within Cloud Run's stateless model:
> - **Postgres `LISTEN`/`NOTIFY`** via `sqlx::postgres::PgListener` — what's the practical fanout cap, latency, recovery semantics under instance restart?
> - **Pub/Sub topic per case** — `google-cloud-pubsub` crate maturity in 2026, ordered delivery, cost at our scale (10k events/day / 100 cases).
> - **NATS / Redis Streams** — both have decent Rust clients; what's the operational overhead of adding either to a Cloud Run-only stack?
> - **WebSocket fanout inside an instance** — `axum::extract::ws::WebSocket` + `tokio::sync::broadcast`. Sound choice? Anything better in `axum` 0.9?
>
> Output: a concrete architecture diagram + Rust pseudocode for each option, recommendation with rationale.

---

## 8. Visualization frontier

> The dashboard currently shows: 6 metric cards, a force-directed SVG graph, a friction matrix, a pre-canonical envelope, a 7-panel item list, a session history, a raw-JSON drawer. We want it to feel as polished as a Bloomberg terminal but native to conflict analysis.
>
> Research the visualization stack in 2026 and recommend what to build:
> - **Graph layout**: stay with vanilla SVG + my own force simulation? Move to `Cytoscape.js`? `d3-force-3d`? `sigma.js`? `vis.js`? `kepler.gl`? Trade-offs: bundle size, interactivity, WebGL for 5k+ nodes.
> - **Timeline**: `vis-timeline`, `d3-timeline-chart`, `react-calendar-timeline`. We want Allen-interval visualisation (overlap, meets, before, during) for the Event primitive.
> - **Heatmap**: stay with native HTML table? `d3-heatmap`? `apexcharts`?
> - **Narrative ribbon**: how do other conflict-analysis tools visualize narrative drift over time per actor?
> - **Brief-style export**: rendering ACO → Markdown / PDF / DOCX. `typst` (Rust-native, beautiful) vs LaTeX vs Pandoc vs hand-rolled.
> - **Dashboard primitives**: are we better off with Tailwind + vanilla JS (current) vs Astro + Solid vs Leptos (Rust-WASM)? Bundle-size vs ergonomic-API trade.
>
> Output: per-component recommendation, bundle-size budget, accessibility considerations.

---

## 9. Benchmarking and evaluation

> AGON has no benchmark today. Every extraction looks right but we cannot prove it. The TCGC (TACITUS Conflict Grammar Corpus, spec in our other repo) will eventually provide this; in the meantime we need internal benchmarks.
>
> Research and propose:
> - A minimal evaluation harness in Rust that takes (a) an input chunk, (b) a gold extraction, (c) a system extraction, and returns per-primitive precision/recall/F1 + graph-edit-distance over the typed subgraph.
> - The right structured-output evaluation methodology for ACO — should we use exact-match on canonical-hash IDs, or fuzzy-match with embedding similarity, or a hybrid?
> - How to evaluate "the right contradictions were found" — F1 over unordered claim-pair sets with materiality weighting?
> - Existing Rust eval crates: `divan` (Nicholas Nethercote's micro-bench), `criterion`, anything for ML-style accuracy metrics?
> - Continuous evaluation: how to run AGON nightly against the TCGC sample set in Cloud Build and post the metrics to a chart we can watch over time.
>
> Output: an evaluation playbook + Rust crate names + a one-week implementation plan.

---

## 10. The product question

> Forget the technology for a moment. AGON's user is one of: a mediator preparing for a session, an HR investigator triaging a complaint, a family lawyer reading a deposition, a diplomatic analyst tracking a back-channel, a therapist preparing for a couple's intake. Each user has a different concept of "what just happened" and "what should I do next".
>
> Research and produce: five fictional but realistic "first session" workflows, one per persona, each ≤ 300 words. For each, identify: (a) what data they upload, (b) what they want to see in the first 30 seconds, (c) what they want to take away (file, email, deck, brief, summary), (d) what they will NEVER tolerate (privacy violations, hallucinated quotes, irreversible actions, etc.).
>
> Based on the five workflows, recommend: which features in `BUILDPLAN.md` Sprint 2 (defeasible reasoning, Z3 contradictions, BATNA/ZOPA, abduction, briefs) should be prioritised first and which can be deferred. The answer should be persona-driven, not architecture-driven.

---

*Paste any of these into a research agent. If you get back something useful, drop it into a `research/` directory in this repo. Each prompt is sized to come back with 1500–4000 words of useful synthesis.*
