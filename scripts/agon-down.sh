#!/usr/bin/env bash
# agon-down.sh — scale AGON dev to near-zero cost.
# Preserves: data, secrets, images, Terraform state.
# Stops: Cloud Run instances (min=0), Cloud SQL compute (storage retained).
set -euo pipefail

PROJECT="${AGON_PROJECT:-tacitus-agon-dev}"
REGION="${AGON_REGION:-us-central1}"
SERVICE="${AGON_SERVICE:-agon-dev}"
SQL_INSTANCE="${AGON_SQL:-agon-dev}"

echo "▼ AGON DOWN — project=$PROJECT"
gcloud config set project "$PROJECT" --quiet

echo "▼ Cloud Run: min-instances=0 (scale-to-zero)"
gcloud run services update "$SERVICE" \
  --project="$PROJECT" --region="$REGION" \
  --min-instances=0 \
  --quiet >/dev/null
echo "  ✓ scaled to zero (cold-start on next request)"

echo "▼ Cloud SQL: activation-policy=NEVER (compute stop, storage retained)"
gcloud sql instances patch "$SQL_INSTANCE" \
  --project="$PROJECT" \
  --activation-policy=NEVER \
  --quiet
echo "  ✓ Cloud SQL stopped"

cat <<'EOF'

╔══════════════════════════════════════════════════════════════╗
║  AGON is DOWN                                                ║
╠══════════════════════════════════════════════════════════════╣
║  Residual cost: ~$0.20–0.50/day                              ║
║  (Cloud SQL storage + GCS + Artifact Registry — no compute)  ║
║                                                              ║
║  To restart:  bash scripts/agon-up.sh                        ║
║  To nuke ALL: bash scripts/agon-nuke.sh   (destroys infra)   ║
╚══════════════════════════════════════════════════════════════╝
EOF
