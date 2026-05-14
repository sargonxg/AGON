#!/usr/bin/env bash
# agon-up.sh — turn the AGON dev environment ON.
# Idempotent. Safe to run multiple times.
set -euo pipefail

PROJECT="${AGON_PROJECT:-tacitus-agon-dev}"
REGION="${AGON_REGION:-us-central1}"
SERVICE="${AGON_SERVICE:-agon-dev}"
SQL_INSTANCE="${AGON_SQL:-agon-dev}"
MIN_INSTANCES="${AGON_MIN_INSTANCES:-1}"

echo "▶ AGON UP — project=$PROJECT region=$REGION"
gcloud config set project "$PROJECT" --quiet

echo "▶ Cloud SQL: ensure RUNNING"
state=$(gcloud sql instances describe "$SQL_INSTANCE" --project="$PROJECT" --format='value(state)')
policy=$(gcloud sql instances describe "$SQL_INSTANCE" --project="$PROJECT" --format='value(settings.activationPolicy)')
if [ "$policy" != "ALWAYS" ]; then
  echo "  → activating Cloud SQL ($state, policy=$policy)"
  gcloud sql instances patch "$SQL_INSTANCE" --project="$PROJECT" --activation-policy=ALWAYS --quiet
else
  echo "  ✓ Cloud SQL already ALWAYS ($state)"
fi

echo "▶ Cloud Run: set min-instances=$MIN_INSTANCES"
gcloud run services update "$SERVICE" \
  --project="$PROJECT" --region="$REGION" \
  --min-instances="$MIN_INSTANCES" \
  --quiet >/dev/null
url=$(gcloud run services describe "$SERVICE" --project="$PROJECT" --region="$REGION" --format='value(status.url)')

echo "▶ Smoke test"
if curl -fsS "${url}/healthz" >/dev/null; then
  echo "  ✓ /healthz OK"
else
  echo "  ✗ /healthz failed — service may still be starting"
fi

cat <<EOF

╔══════════════════════════════════════════════════════════════╗
║  AGON is UP                                                  ║
╠══════════════════════════════════════════════════════════════╣
║  URL:   $url
║  Daily cost (active): ~\$3–8 + per-Vertex-call               ║
║  To stop:  bash scripts/agon-down.sh                         ║
║  Status:   bash scripts/agon-status.sh                       ║
╚══════════════════════════════════════════════════════════════╝
EOF
