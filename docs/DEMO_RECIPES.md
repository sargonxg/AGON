# AGON — Demo Recipes

Copy-paste tests for every capability. Designed for someone with `curl` + `jq` + a terminal.

**Live URL:** `https://agon-dev-tbryoen6qa-uc.a.run.app`
**Auth:** Basic `AGON / AGON` (dev only)
**Required:** `curl`, `jq` (optional but recommended)

Set these once:

```bash
BASE=https://agon-dev-tbryoen6qa-uc.a.run.app
AUTH="AGON:AGON"
```

---

## 0. Is it on?

```bash
curl -s $BASE/healthz                 # 200 OK
curl -s -u $AUTH $BASE/api/info | jq  # backend, db, project, region, version
```

If you get an SSL/connection error, the service may be scaled to zero. Run
`bash scripts/agon-up.sh` from the repo root to bring it up.

---

## 1. Introspection — what's running

```bash
# Full backend state.
curl -s -u $AUTH $BASE/api/system | jq

# Compact view.
curl -s -u $AUTH $BASE/api/system | jq '{
  version: .version,
  patterns: [.patterns_registered[] | select(.live) | .id],
  ml: .ml,
  layers: [.perception_layers[] | "\(.id) \(.name) \(.status)"]
}'
```

Expected: lists 3 live patterns (darvo, anchoring, conspicuous_absence), 8 perception layers, Gemini Flash + Pro as the LLM strategy.

---

## 2. Pattern catalog

```bash
curl -s -u $AUTH $BASE/api/patterns \
  | jq '.patterns[] | {id, version, live, kind: (.kind // "n/a"), public_name}'
```

Expected (today): 5 entries — 3 live + 2 planned.

---

## 3. Pipeline map

```bash
curl -s -u $AUTH $BASE/api/pipeline \
  | jq '.stages[] | "\(.order) \(.id) (\(.crate), \(.kind)) p50=\(.p50_ms)ms"'
```

Expected: ordered list of 12 stages from `ingest` to `persist`.

---

## 4. Perceive — the textbook DARVO

```bash
curl -s -u $AUTH -X POST $BASE/api/perceive \
  -H "Content-Type: application/json" -d '{
    "text": "Sam (Mon): We agreed you own the Q4 deck by Thursday.\nAlex (Mon): Sounds good.\nAlex (Thu): I never said I would own it.\nSam (Thu): That is not what we discussed.\nAlex (Thu): You are putting words in my mouth.",
    "title": "Q4 deck dispute"
  }' \
  | jq '.patterns_detected'
```

Expected: **DARVO** match with confidence 0.70, evidence `["I never", "You're putting words in my mouth"]`, actors `[actor_alex]`.

---

## 5. Perceive — the anchoring negotiation

```bash
curl -s -u $AUTH -X POST $BASE/api/perceive \
  -H "Content-Type: application/json" -d '{
    "text": "Vendor: Our list price is $50,000.\nBuyer: We can do $48,000.\nVendor: Let us settle at $52,000.",
    "title": "anchoring demo"
  }' \
  | jq '.patterns_detected'
```

Expected: **Anchoring** match with confidence 0.75, evidence `["$50,000","$48,000","$52,000"]`.

---

## 6. Perceive — conspicuous absence (the killer pattern)

```bash
curl -s -u $AUTH -X POST $BASE/api/perceive \
  -H "Content-Type: application/json" -d '{
    "text": "Board minutes, Q1 meeting. The Chair opened. Director Park gave an overview of market conditions. Director Ito noted positive trends. The Chair thanked attendees and closed the meeting.",
    "title": "minutes with no commitments"
  }' \
  | jq '.patterns_detected[] | {id: .pattern_id, conf: .raw_confidence, why: .explanation}'
```

Expected: **conspicuous_absence** firing twice — once `structural` (no commitments) at 0.80, once `soft` (no events) at 0.50. Names what is **missing**, not what is present.

---

## 7. Combo — anchoring + DARVO in one perception

```bash
curl -s -u $AUTH -X POST $BASE/api/perceive \
  -H "Content-Type: application/json" -d '{
    "text": "Vendor (Mon): Our list price is $50,000.\nBuyer (Mon): We can do $48,000.\nVendor (Tue): I never said the price was flexible.\nBuyer (Tue): That is not what I heard.\nVendor (Tue): You are putting words in my mouth. Let us settle at $51,000.",
    "title": "combo"
  }' \
  | jq '.patterns_detected[] | {id: .pattern_id, conf: .raw_confidence, evidence: .evidence_excerpts}'
```

Expected: both DARVO (vendor flips after denying) and Anchoring (3 prices within ±30% of $50k) fire from the same input.

---

## 8. Detailed extraction — what Gemini found, end to end

```bash
curl -s -u $AUTH -X POST $BASE/api/perceive \
  -H "Content-Type: application/json" -d '{
    "text": "Sam: We agreed Thursday. Alex: I never agreed.",
    "title": "minimal demo"
  }' \
  | jq '{
      elapsed_ms,
      model,
      input_tokens,
      output_tokens,
      actors: [.extraction.actors[] | {id, label}],
      claims: [.extraction.claims[] | {id, asserter: .actor_id, text, polarity}],
      contradictions: [.extraction.contradictions[]? | {claim_a, claim_b, materiality}],
      patterns: [.patterns_detected[] | {id: .pattern_id, conf: .raw_confidence}],
      friction: .friction_matrix.pairs[0],
      gates: .quality_gates,
      review_questions: .review_questions
    }'
```

---

## 9. Past sessions

```bash
# Most recent 50 perceptions.
curl -s -u $AUTH $BASE/api/sessions | jq '.sessions[] | {id, created_at, friction_score, n_patterns}' | head -30

# A specific session by id.
SID=<paste-id-here>
curl -s -u $AUTH $BASE/api/sessions/$SID | jq .

# Markdown audit report.
curl -s -u $AUTH $BASE/api/sessions/$SID/report.md
```

---

## 10. Streaming perception (SSE)

```bash
curl -N -u $AUTH -X POST $BASE/api/perceive/stream \
  -H "Content-Type: application/json" \
  -d '{"text": "Sam: Yes. Alex: No.", "title": "stream demo"}'
```

You'll see events: `validating`, `auth`, `calling_vertex`, `vertex_done`, `parsing`, `persisting`, `result`.

---

## 11. Web workbench

Browser: <https://agon-dev-tbryoen6qa-uc.a.run.app>

Login: `AGON` / `AGON`. Paste a multi-turn dispute. Click *Perceive*. Watch the friction matrix, force-directed graph, named patterns, and JSON inspector update.

---

## 12. Pause the service to stop billing

From the repo root, on your machine (needs `gcloud` authed to `tacitus-agon-dev`):

```bash
bash scripts/agon-down.sh    # Cloud SQL stops, Cloud Run scales to zero
bash scripts/agon-up.sh      # bring back when you want to demo again
bash scripts/agon-status.sh  # show current state + URLs
```

Cost while down: ~$0.20/day. Cost while up + lightly used: ~$3–8/day.

---

*Maintainer: Giulio Catanzariti · giuliocatanzariti@gmail.com · TACITUS*
