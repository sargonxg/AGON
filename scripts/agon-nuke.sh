#!/usr/bin/env bash
# agon-nuke.sh — DESTROY all dev infrastructure via Terraform.
# This deletes: Cloud Run, Cloud SQL (and data), buckets, secrets, IAM bindings.
# Does NOT delete: GCP project itself, Artifact Registry images, TF state bucket.
set -euo pipefail

PROJECT="${AGON_PROJECT:-tacitus-agon-dev}"
TF_DIR="${AGON_TF_DIR:-infra/terraform/envs/dev}"

cat <<EOF
╔══════════════════════════════════════════════════════════════╗
║                      ⚠  DESTRUCTIVE                          ║
╠══════════════════════════════════════════════════════════════╣
║  Project: $PROJECT
║  This will run: terraform destroy in $TF_DIR
║  It will delete Cloud Run, Cloud SQL (DATA LOST), buckets,   ║
║  secrets, IAM bindings.                                      ║
║                                                              ║
║  Use scripts/agon-down.sh instead if you just want to stop.  ║
╚══════════════════════════════════════════════════════════════╝
EOF

read -p "Type 'NUKE' to proceed: " confirm
[ "$confirm" = "NUKE" ] || { echo "Aborted."; exit 1; }

cd "$TF_DIR"
terraform init -upgrade
terraform destroy -auto-approve

echo "✓ Infrastructure destroyed. TF state preserved for rebuild."
