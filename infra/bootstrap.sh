#!/usr/bin/env bash
set -euo pipefail
source .env

echo "[bootstrap] Project: $GCP_PROJECT_ID, region: $GCP_REGION, env: $ENV"

# Enable APIs (idempotent)
gcloud services enable \
    run.googleapis.com \
    sqladmin.googleapis.com \
    aiplatform.googleapis.com \
    storage.googleapis.com \
    secretmanager.googleapis.com \
    cloudbuild.googleapis.com \
    artifactregistry.googleapis.com \
    eventarc.googleapis.com \
    pubsub.googleapis.com \
    cloudscheduler.googleapis.com \
    servicenetworking.googleapis.com \
    vpcaccess.googleapis.com \
    logging.googleapis.com \
    cloudtrace.googleapis.com \
    monitoring.googleapis.com \
    cloudresourcemanager.googleapis.com \
    iam.googleapis.com \
    --project="$GCP_PROJECT_ID"

# Create Terraform state bucket (idempotent)
TF_BUCKET="${GCP_PROJECT_ID}-terraform-state"
if ! gcloud storage buckets describe "gs://${TF_BUCKET}" --project="$GCP_PROJECT_ID" >/dev/null 2>&1; then
    gcloud storage buckets create "gs://${TF_BUCKET}" \
        --location="$GCP_REGION" \
        --project="$GCP_PROJECT_ID" \
        --uniform-bucket-level-access
    gcloud storage buckets update "gs://${TF_BUCKET}" --versioning --project="$GCP_PROJECT_ID"
fi

# Grant Cloud Build SA the roles it needs (initial bootstrap; refined later by Terraform)
PROJECT_NUMBER=$(gcloud projects describe "$GCP_PROJECT_ID" --format='value(projectNumber)')
CB_SA="${PROJECT_NUMBER}@cloudbuild.gserviceaccount.com"
for role in roles/run.admin roles/iam.serviceAccountUser roles/artifactregistry.writer \
            roles/cloudsql.client roles/secretmanager.secretAccessor roles/storage.objectAdmin; do
    gcloud projects add-iam-policy-binding "$GCP_PROJECT_ID" \
        --member="serviceAccount:${CB_SA}" \
        --role="${role}" \
        --condition=None \
        --quiet
done

# Update .env with bucket name
if ! grep -q "^TF_BUCKET_NAME=" .env || grep -q "^TF_BUCKET_NAME=$" .env; then
    if [ "$(uname)" = "Darwin" ]; then
        sed -i.bak "s|^TF_BUCKET_NAME=.*|TF_BUCKET_NAME=${TF_BUCKET}|" .env
    else
        sed -i "s|^TF_BUCKET_NAME=.*|TF_BUCKET_NAME=${TF_BUCKET}|" .env
    fi
fi

echo "[bootstrap] Done. Next: cd infra/terraform/envs/$ENV && terraform init && terraform apply"
