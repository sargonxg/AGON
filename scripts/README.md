# AGON Control Scripts

One-button start/stop for the dev environment.

## The buttons

| Command | What it does | Time | Cost after |
|---|---|---|---|
| `bash scripts/agon-up.sh` | Cloud Run min=1, Cloud SQL RUNNING, smoke test | ~30s | ~$3–8/day active |
| `bash scripts/agon-down.sh` | Cloud Run min=0, Cloud SQL STOPPED | ~10s | ~$0.20–0.50/day |
| `bash scripts/agon-status.sh` | Show current state | ~5s | — |
| `bash scripts/agon-nuke.sh` | `terraform destroy` — DELETES everything incl. data | ~3min | $0 |

PowerShell wrappers (`.ps1`) call the bash versions via Git Bash.

## Typical flow for "test for a few days"

```bash
bash scripts/agon-up.sh        # Monday morning
# ... build, test, demo ...
bash scripts/agon-down.sh      # Thursday evening
```

That keeps the bill under **$30 total** for the active days, and **<$2/week** when idle.

## Environment overrides

```bash
AGON_PROJECT=tacitus-agon-prod bash scripts/agon-up.sh
AGON_MIN_INSTANCES=2 bash scripts/agon-up.sh
```

Defaults: `tacitus-agon-dev` / `us-central1` / `agon-dev` / `min=1`.

## What up/down does NOT touch

- Artifact Registry images (cents/month, keep them — they're slow to rebuild)
- GCS bucket contents (your data + audit logs)
- Secret Manager secrets
- Terraform state
- IAM bindings

For a full wipe, use `agon-nuke.sh` (rebuilds via `terraform apply` from the env dir).
