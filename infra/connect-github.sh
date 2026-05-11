#!/usr/bin/env bash
set -euo pipefail
source .env

echo "[ci-connect] Connect GitHub repo ${GITHUB_OWNER}/${GITHUB_REPO} to Cloud Build in project ${GCP_PROJECT_ID}."
echo ""
echo "Step 1 — Install the Google Cloud Build GitHub App on your account/org:"
echo "  https://github.com/apps/google-cloud-build"
echo "  Grant it access to ${GITHUB_OWNER}/${GITHUB_REPO}."
echo ""
echo "Step 2 — Link the repository to this GCP project (opens browser):"
gcloud builds connections create github "github-${GITHUB_OWNER}" \
    --region="$GCP_REGION" \
    --project="$GCP_PROJECT_ID" || true

echo ""
echo "Step 3 — Authorize (interactive):"
echo "  gcloud builds connections describe github-${GITHUB_OWNER} --region=${GCP_REGION} --project=${GCP_PROJECT_ID}"
echo "  Follow the authUri printed and complete the OAuth flow in the browser."
echo ""
echo "Step 4 — Create the repository link:"
gcloud builds repositories create "${GITHUB_REPO}" \
    --connection="github-${GITHUB_OWNER}" \
    --remote-uri="https://github.com/${GITHUB_OWNER}/${GITHUB_REPO}.git" \
    --region="$GCP_REGION" \
    --project="$GCP_PROJECT_ID" || true

echo ""
echo "Step 5 — Create the build trigger (runs on push to main):"
gcloud builds triggers create github \
    --name="agon-${ENV}-main" \
    --repository="projects/${GCP_PROJECT_ID}/locations/${GCP_REGION}/connections/github-${GITHUB_OWNER}/repositories/${GITHUB_REPO}" \
    --branch-pattern="^main$" \
    --build-config="infra/cloudbuild.yaml" \
    --region="$GCP_REGION" \
    --project="$GCP_PROJECT_ID" || true

echo "[ci-connect] Done. Verify in console: https://console.cloud.google.com/cloud-build/triggers"
