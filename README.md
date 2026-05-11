```
   █████╗  ██████╗  ██████╗ ███╗   ██╗
  ██╔══██╗██╔════╝ ██╔═══██╗████╗  ██║
  ███████║██║  ███╗██║   ██║██╔██╗ ██║
  ██╔══██║██║   ██║██║   ██║██║╚██╗██║
  ██║  ██║╚██████╔╝╚██████╔╝██║ ╚████║
  ╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝
              a perception engine for human conflict.
              rust ─ vertex ai ─ cloud sql ─ cloud run.
```

> Drop fifty pages of real human text — depositions, messages, transcripts, complaints, journals — and AGON drives through it like a self-driving car drives through a street. Eight typed sensors. One canonical world model. Every conclusion auditable to a verbatim quote.

```
                 ┌─────────────────────┐
   you  ─text─→  │  Vertex AI Gemini   │  ─ json ─┐
                 │  (typed sensor)     │           │
                 └─────────────────────┘           ▼
                                          ┌───────────────────┐
                                          │   ACO ontology    │  ←─ rust types
                                          │  8 primitives     │     blake3 hash
                                          │  + edges          │     provenance
                                          └────────┬──────────┘
                                                   ▼
                                          ┌───────────────────┐
                                          │   inference       │  datalog · z3 · lp
                                          │   scoring         │  friction · risk · trust
                                          │   patterns        │  darvo · gaslighting · 4hm
                                          └────────┬──────────┘
                                                   ▼
                                          ┌───────────────────┐
                                          │   cloud sql       │  pgvector · ltree
                                          │   pg 16 · private │  audit log · alias graph
                                          └───────────────────┘
```

```
  live:  https://agon-dev-tbryoen6qa-uc.a.run.app
  docs:  ARCHITECTURE.md · BUILDPLAN.md · SETUP.md
  by:    tacitus.me
```

---

## what it sees

```
input:   "Sam: We agreed Monday — you own the deck.
          Alex: Sounds good.
          Alex (Thu): I never said I'd own it. Just help."

output:  478 → 1082 tokens · 8.9s · gemini-2.5-flash · 121 tok/s
         actors:        Sam ⊥ Alex
         claims:        6  (1 deny, 5 assert)
         events:        4  (monday agreement, jenkins pitch, thu deadline, thu denial)
         commitments:   2  (own deck — contested; design — accepted)
         contradictions:1  (material — Sam asserts agreement, Alex denies)
         patterns:      1  (defensiveness, confidence 0.80)
         friction:      80 / 100
         persisted:     cloud sql · 763cdb18-7274-42a9-8a3b-ab4b456370d3
```

every primitive cites its source span. nothing paraphrased. reasoning is auditable by clicking.

---

## the aco ontology

eight primitives, locked. nineteen edge types. no rdf. no triples. just rust.

```
ACTOR        individuals · organizations · states · coalitions
CLAIM        asserted fact / evaluation / normative statement
INTEREST     underlying goal motivating positions (fisher/ury)
CONSTRAINT   rule / norm / structural limit
LEVERAGE     resource / dependency / capability
COMMITMENT   promised future action (subject + deadline + status)
EVENT        dated or orderable occurrence
NARRATIVE    coherent framing across claims, per actor
                + PatternFinding (darvo, gaslighting, 4 horsemen)
                + AffectMarker, RepairAttempt, BidForConnection
```

---

## architecture

```
   browser ──┬─→ cloud run (axum + embedded dashboard, rust)  ─→  vertex ai
              │  /api/perceive/stream    SSE per-stage timing
              └─→ /api/sessions          persistence, history
                            │
                            └─→ cloud sql (postgres 16 · pgvector)
```

cloud-native from day 1. no laptop runtime. no api keys. service-account auth via metadata server.

| layer        | implementation                                       |
|--------------|------------------------------------------------------|
| compute      | cloud run · 1 vcpu / 1 gib · scale-to-zero           |
| storage      | cloud sql postgres 16 · pgvector · ltree · private ip|
| llm          | vertex ai gemini 2.5 flash + pro · schema-constrained|
| ingest       | gcs signed urls → eventarc → cloud run               |
| cognition    | rust in-process · ascent · z3 · good_lp              |
| ci/cd        | cloud build · artifact registry · `gcloud builds submit`|
| iac          | terraform · 32 resources · 8 modules                 |

---

## the crates

```
crates/
  aco-core/        8 primitives · blake3 hash · provenance · fol · proptest
  aco-llm/         vertex ai client · mock backend · cost ledger
  aco-embed/       fastembed + hnsw (next)
  aco-storage/     sqlx · sessions table · migrations
  aco-perceive/    parallel extractors (next)
  aco-fuse/        canonicalization + alias graph (next)
  aco-infer/       datalog · z3 · lp · abduction (next)
  aco-score/       friction · risk · power · trust · repair (next)
  aco-learn/       correction log · active learning (next)
  aco-server/      axum + embedded dashboard + sse  ← live on cloud run
  aco-cli/         `agon` thin client (next)
  aco-bench/       criterion benches
```

---

## quickstart

you need: a gcp project with billing, `gcloud`, `terraform`, `make`, git bash on windows.

```bash
git clone https://github.com/sargonxg/AGON.git && cd AGON
cp .env.example .env                                   # GCP_PROJECT_ID, GCP_REGION, ENV
make bootstrap                                         # 17 apis + tf state bucket + cb sa iam
make infra-apply                                       # 32 resources, ~8 min
gcloud builds submit --tag IMG && gcloud run deploy ...
make url                                               # → https://...run.app
```

full walkthrough in [SETUP.md](./SETUP.md).

---

## cost (dev tier)

```
cloud run service       $0 idle · ~$0.10–0.50 per demo-hour
cloud sql db-f1-micro   ~$10–15 / mo
cloud storage 10 gb     ~$0.20  / mo
artifact registry 2 gb  ~$0.20  / mo
vertex gemini 2.5 flash ~$0.05–0.50 / demo
                        ────────────────────
total                   ~$15–25 / mo + per-use gemini
```

eu residency: flip one tfvars line (`region = "europe-west4"`).

---

## why rust

```
  ┌─ schema-constrained extraction
  │     vertex ai is the typed sensor. JSON in, JSON out. rust validates.
  │
  ├─ canonicalization (alias graph, dedup)
  │     petgraph + blake3 + hnsw_rs. no Python startup tax. zero-copy.
  │
  ├─ deductive inference (ascent / datalog)
  │     parallel closure on 50k primitives in < 1s. one binary.
  │
  ├─ contradiction detection (z3)
  │     real SMT solver. unsat cores trace back to source spans.
  │
  ├─ negotiation modelling (good_lp · lp + milp)
  │     batna / zopa as feasibility problems. solver in-process.
  │
  ├─ pattern recognition (rule-based + neural, both)
  │     fastembed for embeddings. candle / burn for in-binary neural inference.
  │
  └─ sovereign reasoning
        no remote inference service besides extraction.
        deterministic. reproducible. auditable.
```

---

## citations

gottman & levenson (2000) · karpman (1968) · freyd (1996) · bowen (1978)
modgil & prakken (aspic+, 2014) · dung (acceptability, 1995)
allen (intervals, 1983) · searle (speech acts, 1969) · grice (1975)
fisher & ury (1981) · sahebolamri et al. (ascent, oopsla 2023)

full bibliography in [ARCHITECTURE.md §20](./ARCHITECTURE.md#20-citations).

---

```
  built by tacitus            license: tbd · all rights reserved
  giulio catanzariti          giuliocatanzariti@gmail.com
  ──────────────────────────────────────────────────────────
  conflict is legible.
  perception is sovereign.
```
