<div align="center">

# AGON

**A Tesla-style perception engine for human conflict.**

*Drop in 50 pages of real text — messages, transcripts, complaints, depositions, journals — and AGON drives through it like a self-driving car drives through a street. It sees people, events, claims, frictions, and patterns. It deduplicates everything. It scores the relationships. It tells you where the conflict is, who has leverage, what's escalating, and what's about to break.*

*Built in Rust. Stores in Postgres. Visualizes natively. Learns from your corrections. Sovereign reasoning, not a chat wrapper.*

[![Built by TACITUS](https://img.shields.io/badge/built%20by-TACITUS-black)](https://tacitus.me)
[![Rust 1.78+](https://img.shields.io/badge/rust-1.78%2B-orange)](https://www.rust-lang.org)
[![Postgres 16+](https://img.shields.io/badge/postgres-16%2B-blue)](https://www.postgresql.org)

</div>

---

## The use case in one paragraph

You have a folder. Inside it: a 50-page divorce filing, three years of WhatsApp messages between two co-parents, a 12-page HR complaint, two depositions, a mediator's notes, and a journal. You want, within five minutes:

- A canonical list of every person involved, deduplicated across all documents
- A timeline of every event mentioned, fused across sources
- A friction score for every relationship dyad
- A risk score for escalation, with the specific signals driving it
- The Four Horsemen count per person (Gottman: criticism, contempt, defensiveness, stonewalling)
- The unbacked commitments, the contradictions across time, the leverage imbalance
- A live, clickable visualization where every node traces back to the verbatim quote that grounds it
- An exportable brief in the format you need (legal, mediator-prep, HR, therapist-prep)

That is what AGON does.

---

## How it works — the Tesla metaphor

A Tesla doesn't read the road. It *perceives* it — eight cameras feeding parallel networks (object detection, depth, motion prediction), then a fusion layer that turns those streams into a single coherent world model, then planning, then control. Each layer is fast, the layers run concurrently, and the system learns from every mile driven.

AGON is the same shape, applied to text.

```mermaid
flowchart LR
    INGEST[Ingest<br/>50 pages of mixed text]
    PERCEPT[Perception<br/>parallel extractors]
    FUSION[Fusion<br/>canonicalization + dedup]
    WORLD[World Model<br/>Postgres + pgvector + petgraph]
    COGNITION[Cognition<br/>Datalog + SMT + LP + scoring]
    SURFACE[Surface<br/>live viz + scores + briefs]
    LEARN[Learning<br/>corrections become signal]

    INGEST --> PERCEPT --> FUSION --> WORLD --> COGNITION --> SURFACE
    SURFACE -.->|user labels| LEARN
    LEARN -.->|refines| PERCEPT
    LEARN -.->|recalibrates| COGNITION
```

| Tesla | AGON |
|---|---|
| 8 cameras | Parallel extractors (entities, events, claims, affect, patterns, time) |
| Object detection per frame | Per-chunk Gemini extraction with constrained JSON schema |
| Sensor fusion | Pre-canonical signatures + entity resolution + temporal alignment |
| HD map + world model | Postgres tables + in-memory petgraph + pgvector |
| Path planning | Datalog inference, Z3 contradiction, LP for negotiation |
| Control output | Friction scores, risk projections, visual dashboard, briefs |
| Fleet learning | Active learning from user corrections; pattern detector retraining |

The unit of throughput is the *frame* — a chunk of text. Each frame is perceived in parallel by multiple specialized extractors. Their outputs are *fused* into the world model only after passing canonicalization. Inference runs continuously over the world model. The dashboard updates as state changes.

---

## What's new vs. a generic LLM analysis

| Capability | Generic LLM | AGON |
|---|---|---|
| Read 50 pages | Yes | Yes |
| Summarize | Yes | Yes (and as a graph-derived brief, not a paraphrase) |
| Identify people across documents with name variations | Inconsistent | Deterministic canonicalization + embedding-based entity resolution |
| Deduplicate events that two documents describe differently | Almost never | Pre-canonical signatures + LLM tiebreaker on fuzzy matches |
| Track friction over time as a trajectory | No | Time-indexed scoring with per-event audit trail |
| Spot a contradiction between something said in month 1 and month 14 | If lucky | Z3 unsat-core with both quotes surfaced |
| Detect Gottman's Four Horsemen | No | Native pattern extractor with confidence |
| Score escalation risk with stated drivers | No | Calibrated risk model with feature attribution |
| Produce reproducible output | No (temperature noise) | Yes (same input + prompt version → same output) |
| Be auditable from conclusion → quote | No | Every score has a derivation chain back to spans |
| Run locally on a laptop | No | Yes |
| Cost per 50-page run | Pay-per-token | Around **$0.10–$0.40** in Gemini Flash calls |

---

## What you get out — concrete

For a real folder of 50 pages of mixed conflict text, AGON produces:

### A canonical entity ledger
Every person who appears in any document, deduplicated. "Sarah Chen", "Sarah", "my wife", "Ms. Chen", "@sarah" — collapsed into one canonical actor with all aliases recorded. Same for organizations, places, and recurring events.

### A typed event timeline
Every event the documents reference, ordered, with thematic roles filled (agent, patient, witness). Events mentioned in multiple documents are fused into a single canonical event with all sources cited.

### A friction map
For every dyad of actors who interact in the corpus:
- **Friction Score** (0–100): composite of conflict frequency, severity, asymmetry, and repair deficit
- **Risk Score** (0–100): probability of rupture or escalation in the next *N* days based on trajectory features
- **Power Asymmetry** (-1 to +1): leverage imbalance with mechanism breakdown
- **Trust Trajectory**: a time-series line tracking commitment-fulfillment ratio
- **Four Horsemen Counts**: criticism, contempt, defensiveness, stonewalling — per person, per dyad
- **Repair Capacity**: ratio of de-escalation attempts to escalation events
- **Bid/Turn Ratio**: bids for connection vs turning-away or turning-against responses

### A pattern report
Detected behavioural patterns with confidence and supporting evidence:
- DARVO sequences (Deny, Attack, Reverse Victim and Offender)
- Gaslighting markers (reality denial, memory-undermining language)
- Triangulation (using third parties as leverage)
- Stonewalling episodes (withdrawal of engagement)
- Repair attempts that landed and ones that didn't
- Escalation cycles with their typical duration

### A contradiction log
Every place an actor said one thing in document A and a contradictory thing in document B, with both verbatim quotes surfaced side by side.

### A gap report
What's conspicuously missing: commitments without verification, claims without supporting evidence, expected responses that never came, third parties who should have weighed in but didn't.

### A live visualization
A browser-based dashboard with synchronized views:
- **Graph view**: actors, events, claims, narratives as a force-directed graph; node shape by primitive type, edge thickness by salience, colour by friction intensity
- **Timeline view**: events on a synchronised horizontal axis with claim density overlay
- **Friction heatmap**: actor × actor matrix coloured by friction score, click to drill into the dyad
- **Score dashboard**: live cards for each dyad with score breakdowns and trajectories
- **Evidence pane**: any selected primitive shows its verbatim spans across all source documents

### Exportable briefs
Generated from the world model, not from re-prompting the LLM:
- **Mediator's prep brief**: each party's stated and inferred interests, BATNA, ZOPA, suggested opening moves
- **Legal brief**: chronology, statement of issues, points of contradiction with citations
- **HR brief**: pattern findings with evidence, recommended interventions, risk assessment
- **Therapist's prep brief**: relational dynamics, attachment patterns, repair-attempt analysis
- **Custom brief**: write your own template; AGON fills it from the graph

---

## Storage — one database, the right one

The previous instinct was "graph database for graph data." The pragmatic instinct is **Postgres**. AGON ships with a single Postgres-based storage layer because:

- One database to operate, back up, monitor, scale
- `pgvector` for embedding similarity at production speed
- `ltree` for narrative-claim hierarchies
- `JSONB` for the long tail of attributes
- **Apache AGE** extension as the optional Cypher layer — graph queries when you want them, all inside Postgres
- Mature replication, point-in-time recovery, the works

The in-memory `petgraph::StableGraph` exists only as a hot working set for the inference engine. The source of truth is Postgres. Every query an analyst runs goes through Postgres. Every external integration reads from Postgres. The graph database is *optional* — turn on Apache AGE if you want Cypher queries, leave it off if you don't.

| Storage role | Component |
|---|---|
| Source of truth | Postgres 16+ |
| Embeddings + ANN | `pgvector` 0.8+ |
| Graph queries (optional) | Apache AGE 1.5+ |
| Hot working set | `petgraph::StableGraph` in-memory |
| Audit log | Postgres append-only table + WAL |
| Analytical export | Parquet / DuckDB |
| Hierarchies | `ltree` |
| Flexible attributes | `JSONB` columns |

---

## Quickstart — run AGON locally in 10 minutes

### Prerequisites

| Requirement | How to get it |
|---|---|
| Rust 1.78+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Postgres 16+ with `pgvector` | macOS: `brew install postgresql@16 pgvector` · Linux: `apt install postgresql-16 postgresql-16-pgvector` |
| Z3 SMT solver | macOS: `brew install z3` · Linux: `apt install libz3-dev` |
| Gemini API key | Free tier at [aistudio.google.com/apikey](https://aistudio.google.com/apikey) |

### Install

```bash
git clone https://github.com/sargonxg/AGON.git
cd AGON
cargo build --release
cargo install --path crates/aco-cli      # installs `agon` on PATH
```

### Configure

```bash
cp .env.example .env
# Edit .env:
#   GEMINI_API_KEY=...
#   AGON_DATABASE_URL=postgres://agon:password@localhost:5432/agon
#   AGON_DATA_DIR=./data
```

### Initialize the database

```bash
agon db init
# → Creates database, runs migrations, installs pgvector + ltree extensions
# → Optional: agon db init --enable-age   to install Apache AGE for graph queries
```

### Drop your folder in and watch AGON drive

```bash
# 1. Ingest a whole folder at once
agon ingest ./my_case/         # all .pdf .docx .txt .md .eml .csv in the folder
# → 12 documents ingested, 213 chunks planned

# 2. Run perception (parallel extractors)
agon perceive --concurrency 8
# → Entities extractor: 47 raw actors found
# → Events extractor:   89 raw events found
# → Claims extractor:  312 raw claims found
# → Affect extractor:  written sentiment per turn
# → Patterns extractor: 23 Four-Horsemen candidates, 4 DARVO candidates

# 3. Run fusion (canonicalization)
agon fuse
# → 47 raw actors → 14 canonical actors (33 alias merges)
# → 89 raw events → 61 canonical events (28 coreferent merges)
# → 312 raw claims → 287 canonical claims (25 duplicate suppressions)

# 4. Run cognition (inference + scoring)
agon think
# → 142 deductive facts derived
# → 18 gaps detected (8 unbacked commitments, 6 silent actors, 4 agency gaps)
# → 7 contradictions found
# → Friction scored for 31 dyads
# → Risk scored for top 8 dyads
# → 4 pattern findings confirmed (DARVO ×2, gaslighting ×1, triangulation ×1)

# 5. Open the live dashboard
agon viz --open
# → http://localhost:7878
# → Graph view + timeline + friction heatmap + score cards

# 6. Generate a brief
agon brief --style mediator-prep --dyad "<actor_a>" "<actor_b>" --out brief.md

# 7. Or just ask
agon ask "what are the top 3 risks of escalation in the next 60 days?"
```

---

## The MVP demo recording — the production workflow

The first deliverable is a **5–15 minute screencast** that demonstrates AGON's extensive capabilities on real conflict text. The recording protocol is reproducible — anyone with the repo, a Gemini key, and OBS Studio can produce the same video.

### Pre-flight checklist

- [ ] `cargo build --release` complete
- [ ] Postgres running with `agon db init` done
- [ ] Demo folder ready: `corpora/demo_workplace_dispute/` (synthetic but realistic; checked in)
- [ ] Gemini calls pre-cached so timing is deterministic on camera
- [ ] OBS Studio configured: 1920×1080@60fps; scenes for terminal, browser, split view, full browser
- [ ] Terminal font ≥ 16pt, dark theme, clean prompt
- [ ] `agon serve --port 7878` running in background; browser at `http://localhost:7878`

### Take 1 — the full pipeline (under 90 seconds of compute thanks to caching)

**00:00 — The hook**

> *"This is AGON. Drop fifty pages of real human conflict text into it. In under five minutes you get the friction map, the risk scores, the contradictions, and a live visualization that traces every conclusion back to the verbatim quote. It's not a chatbot. It's a perception engine. Watch."*

**00:30 — Ingest**

```bash
agon ingest corpora/demo_workplace_dispute/
```

Terminal scrolls: each document loaded, hash, size, language. Narrate the variety: emails, Slack export, HR memo, deposition excerpt.

**01:00 — Perception**

```bash
agon perceive --concurrency 8 --verbose
```

Five parallel extractors light up. The browser at `localhost:7878` already shows actor nodes appearing as they are extracted. Narrate the multi-camera analogy: each extractor sees the text through a different lens.

**02:00 — Fusion (the dedup moment)**

```bash
agon fuse --explain
```

The output shows alias merges: "Sarah", "Sarah Chen", "the PM", "Ms. Chen" → one canonical actor. Narrate: *"This is the sensor fusion. Four documents mention the same person in four different ways. AGON resolves it deterministically."*

**03:00 — Cognition (the inference moment)**

```bash
agon think --explain
```

The terminal streams derived facts, gaps, contradictions, scores. In parallel, the browser dashboard pulses as scores update live. Cut to the browser full-screen.

**04:00 — Five wow moments on the dashboard**

1. **The friction heatmap.** Mouse over the hottest dyad. A side panel shows the score breakdown: Four Horsemen counts, repair deficit, contradiction count. Click any score → drill into the events that drove it → click an event → see the verbatim quote.
2. **The contradiction across time.** Click the contradiction badge on an actor. Two quotes appear side by side, eight months apart. The Z3 unsat-core is rendered as a red edge between them.
3. **The DARVO sequence.** A pattern detector flagged a 4-turn DARVO pattern. The dashboard renders the sequence with each turn highlighted in the source document.
4. **The unbacked commitment.** A gap detector found a promise made with no interest evidence backing it. The system abductively proposes three candidate interests, ranked.
5. **The risk trajectory.** A 30-day forward projection of friction score with confidence band. The drivers are listed as feature attribution: "rising contempt count", "no repair attempts in 14 days", "leverage asymmetry widening."

**12:00 — The brief**

```bash
agon brief --style mediator-prep --dyad "Sarah" "James" --out brief.md
bat brief.md
```

A structured mediator's prep brief renders. Every paragraph cites spans. Narrate: *"This is what an analyst hands to a principal. The graph wrote it. The LLM didn't."*

**14:00 — The learning moment**

In the dashboard, click an extraction labelled "DARVO candidate (0.74 confidence)". Mark it as "not DARVO". The system logs the correction. Show `agon learn report` — pending corrections, model recalibration queue.

**14:30 — The close**

> *"AGON ships as a Rust binary. It runs on a laptop. It stores in Postgres. It scales to a cluster. It learns from your corrections. The reasoning is sovereign and reproducible. Every score traces to a quote. This is the substrate for what's next at TACITUS."*

End on `agon stats` showing total primitives, derived facts, costs spent, and per-stage timing.

---

## Capabilities catalog

### Perception (parallel extractors)
- **Entity extractor**: actors with kind (person, org, group), aliases, agency markers
- **Event extractor**: events with thematic roles (Agent, Patient, Witness, Beneficiary, Mediator), time, place
- **Claim extractor**: claims with speech-act class (Searle: Assertive, Directive, Commissive, Expressive, Declaration), modality, stance
- **Affect extractor**: emotional valence per turn, intensity, target actor
- **Pattern extractor**: Four Horsemen, DARVO, gaslighting, triangulation, repair attempts, bids for connection
- **Temporal extractor**: explicit dates, relative time, durations, intervals
- **Commitment extractor**: promises, vows, agreements, with deadlines and verification regimes
- **Interest extractor**: stated needs and desires (inferred interests are added by the abduction layer)

### Fusion (canonicalization)
- **Pre-canonical signatures**: text normalization + Blake3 hash for exact-match dedup
- **Embedding signatures**: `fastembed`-derived dense vectors for fuzzy match
- **Entity resolution**: HNSW ANN search + Gemini tiebreaker on borderline matches
- **Event coreference**: same event mentioned in multiple docs collapsed into one
- **Claim deduplication**: same claim phrased differently collapsed
- **Temporal alignment**: relative times resolved to absolute when possible
- **Alias graph**: every merge preserves the alias for downstream display

### World model (storage)
- Postgres tables for each primitive type
- pgvector indices for embedding queries
- Apache AGE (optional) for Cypher graph queries
- In-memory petgraph for hot inference paths
- Append-only audit log for reproducibility

### Cognition (inference)
- Deductive Datalog (`ascent`): transitive leverage, coalition graphs, narrative ⊆ claims, temporal Allen relations
- Defeasible argumentation (ASPIC+ encoded into stratified Datalog): rebut, undercut, undermine
- SMT contradiction (`z3`): per-actor consistency, deontic conflict, temporal consistency
- LP/MILP (`good_lp` with SCIP): BATNA, ZOPA, mediation moves, coalition stability
- Abduction loop: typed gap → typed Gemini prompt → ranked candidates → defeasible re-injection → re-closure
- Pragmatics: scalar implicature, evasion, non-sequitur, presupposition
- Pattern recognition: Four Horsemen, DARVO, gaslighting, triangulation detection
- Calibrated scoring: friction, risk, power asymmetry, trust trajectory, repair capacity

### Surface (outputs)
- Live HTTP/WebSocket dashboard (Axum + Cytoscape.js + d3)
- Friction heatmap, score dashboard, graph view, timeline view, evidence pane
- Brief generation (mediator-prep / legal / HR / therapist-prep / custom)
- Exports: CSV, JSON, JSON-LD, Parquet, Neo4j Bolt, FalkorDB, Oxigraph RDF
- CLI ask-mode (`agon ask "<question>"`) — natural-language query over the world model

### Learning loop
- User corrections logged with primitive ID and correction type
- Active learning queue surfaces low-confidence items for human review
- Pattern detector retraining triggered when correction count crosses threshold
- Prompt-version pinning so corrections are tied to the extractor version that made the error

---

## Roadmap — beyond the MVP

The MVP (this repo) is the foundation. The phases that turn it into the product anchor of tacitus.me:

### Phase 2 — Decision support (weeks 4–6 post-MVP)
- **Collapse Predictor**: gradient-boosted model on relationship-rupture datasets with ACO-graph features for half-life prediction
- **Counterfactual Sandbox**: edit the graph, re-run closure, see the diff (mediators externalise their mental simulation)
- **Multi-Party Strategist**: n-party coalition stability via Shapley; swing-actor identification

### Phase 3 — Continuous intelligence (weeks 7–9 post-MVP)
- **Wire Watcher**: subscribe to RSS, X, OCHA, ICG, mail accounts; deltas pushed when new docs change the world model
- **Anomaly / Silence Engine**: baseline per-actor speech patterns; flag 5σ deviations
- **Active learning at scale**: a queue of human-labelled corrections becomes a training set for the pattern detector

### Phase 4 — Productisation (weeks 10–14 post-MVP)
- **Multi-tenancy** at the Postgres schema level
- **GraphQL API** so PRAXIS / CONCORDIA / Wind Tunnel can call AGON as a backing service
- **Differential dataflow** under `aco-infer` for live-edit dossiers (replaces batch closure)
- **Probabilistic logic** via Scallop FFI for soft inference under uncertainty
- **Multi-lingual extraction**: Gemini handles natively; prompts in `prompts/i18n/`

---

## Project status

This is **v0.1 in active development**. The MVP scope is captured in [BUILDPLAN.md](./BUILDPLAN.md). The architecture is captured in [ARCHITECTURE.md](./ARCHITECTURE.md). Expect:

- ✅ Eight ACO primitive types + interpersonal-conflict extensions, full provenance
- ✅ Five parallel Gemini-driven extractors (entities, events, claims, affect, patterns)
- ✅ Canonicalization layer with deterministic and embedding-based dedup
- ✅ Postgres + pgvector storage; in-memory petgraph hot path
- ✅ Datalog inference (`ascent`) with 80+ rules
- ✅ Z3 contradiction detection
- ✅ BATNA/ZOPA solver (`good_lp` + SCIP)
- ✅ Abduction loop with bounded cycles
- ✅ Calibrated scoring (friction, risk, power, trust, repair)
- ✅ Pattern detection (Four Horsemen, DARVO, gaslighting, triangulation)
- ✅ Live dashboard (graph + timeline + heatmap + scores)
- ✅ Five demo corpora with golden snapshots
- ✅ Brief generation in four templates
- ✅ Learning loop with correction logging
- 🚧 Pattern detector retraining pipeline (Phase 2)
- 🚧 Continuous monitoring (Phase 3)
- 🚧 Multi-tenancy (Phase 4)

---

## Repository layout

```
AGON/
├── README.md                       # this file
├── ARCHITECTURE.md                 # full technical spec
├── BUILDPLAN.md                    # day-by-day build sequence
├── Cargo.toml                      # workspace
├── crates/
│   ├── aco-core/                   # primitives + provenance + IDs
│   ├── aco-llm/                    # Gemini client + LlmBackend trait
│   ├── aco-embed/                  # fastembed + similarity
│   ├── aco-storage/                # Postgres + pgvector + in-mem petgraph + exports
│   ├── aco-perceive/               # parallel extractors (entities / events / claims / affect / patterns / time)
│   ├── aco-fuse/                   # canonicalization + entity resolution + dedup
│   ├── aco-infer/                  # ascent + z3 + good_lp + abduction
│   ├── aco-score/                  # friction / risk / power / trust / repair scoring
│   ├── aco-learn/                  # correction log + active learning + retraining hooks
│   ├── aco-cli/                    # `agon` binary
│   ├── aco-server/                 # Axum HTTP/WS + dashboard
│   └── aco-bench/                  # criterion benches
├── prompts/                        # versioned Gemini prompts + JSON schemas
├── migrations/                     # sqlx Postgres migrations
├── corpora/                        # golden test corpora (synthetic but realistic)
├── examples/                       # runnable demo scenarios
├── docs/                           # mdbook source
└── .github/workflows/              # CI
```

---

## License

To be determined. See [ARCHITECTURE.md §14](./ARCHITECTURE.md#14-licensing-posture). Until a license file is committed, **AGON is unlicensed and all rights are reserved by TACITUS / Giulio Catanzariti.** Do not redistribute.

---

## Citations

The full bibliography is in [ARCHITECTURE.md §17](./ARCHITECTURE.md#17-citations). Intellectual anchors:

- **Gottman, J.M. & Levenson, R.W.** *The timing of divorce: Predicting when a couple will divorce over a 14-year period.* JMF 2000 — the Four Horsemen and the 5:1 ratio.
- **Karpman, S.** *Fairy tales and script drama analysis.* TAB 1968 — the drama triangle.
- **Modgil, S. & Prakken, H.** *The ASPIC+ framework for structured argumentation.* Argument & Computation 2014.
- **Dung, P.M.** *On the acceptability of arguments.* AIJ 1995.
- **Allen, J.F.** *Maintaining knowledge about temporal intervals.* CACM 1983.
- **Searle, J.R.** *Speech Acts.* CUP 1969 — speech-act taxonomy.
- **Grice, H.P.** *Logic and Conversation.* 1975 — Gricean implicature.
- **Fisher, R. & Ury, W.** *Getting to Yes.* 1981 — BATNA / ZOPA.
- **Freyd, J.J.** *Violations of power, adaptive blindness and betrayal trauma theory.* 1996 — DARVO.
- **Sahebolamri, A. et al.** *Ascent.* OOPSLA 2023.
- **Apache AGE** — graph extension for Postgres.

The ACO ontology is original to TACITUS, derived from operational conflict-analysis practice. It generalises from diplomatic and Security Council work to interpersonal, workplace, and family conflict because the primitives are universal: every conflict has actors, claims, interests, constraints, leverage, commitments, events, and narratives.

---

<div align="center">

*Built by [TACITUS](https://tacitus.me). Drive through text.*

</div>
