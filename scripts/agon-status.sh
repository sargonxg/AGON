#!/usr/bin/env bash
# agon-status.sh — show current AGON state + cost shape.
set -euo pipefail

PROJECT="${AGON_PROJECT:-tacitus-agon-dev}"
REGION="${AGON_REGION:-us-central1}"
SERVICE="${AGON_SERVICE:-agon-dev}"
SQL_INSTANCE="${AGON_SQL:-agon-dev}"

echo "═══ AGON status ═══"
echo "Project: $PROJECT"

echo
echo "── Cloud Run ──"
gcloud run services describe "$SERVICE" --project="$PROJECT" --region="$REGION" \
  --format="value(metadata.name,status.url,spec.template.metadata.annotations.'autoscaling.knative.dev/minScale',spec.template.spec.containers[0].image)" 2>/dev/null \
  | awk -F'\t' '{ printf "  name:   %s\n  url:    %s\n  min:    %s\n  image:  %s\n", $1, $2, $3, $4 }'

echo
echo "── Cloud SQL ──"
gcloud sql instances describe "$SQL_INSTANCE" --project="$PROJECT" \
  --format="value(name,state,settings.activationPolicy,settings.tier)" \
  | awk -F'\t' '{ printf "  name:   %s\n  state:  %s\n  policy: %s\n  tier:   %s\n", $1, $2, $3, $4 }'

echo
echo "── Buckets ──"
gcloud storage buckets list --project="$PROJECT" --format="value(name)" 2>/dev/null | sed 's/^/  /'

echo
echo "── Last 3 Cloud Run revisions ──"
gcloud run revisions list --project="$PROJECT" --region="$REGION" --service="$SERVICE" \
  --limit=3 --format="table(metadata.name,status.conditions[0].lastTransitionTime,spec.containers[0].image.basename())" 2>/dev/null \
  | sed 's/^/  /'

echo
echo "── Billing this month (project-scoped) ──"
echo "  See: https://console.cloud.google.com/billing"
echo "  Or:  gcloud billing accounts list  +  https://console.cloud.google.com/billing/<account>/reports"
