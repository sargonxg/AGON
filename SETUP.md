# AGON — Setup Prompts (Interactive Steps)

This file lists every interactive step required to go from a fresh clone to a deployed Cloud Run revision. Steps marked **[You]** require your hands on a keyboard (auth flows, billing, browser OAuth). Steps marked **[Auto]** can be run by a coding agent or `make` once prerequisites are in place.

Working directory throughout: `C:\Users\giuli\AGON`.

---

## 0. Install local prereqs (one-time, [You])

```powershell
# Terraform
winget install Hashicorp.Terraform

# GNU Make (via chocolatey or scoop). If neither is installed:
#   choco install make
#   - or -
#   scoop install make
# If you have Git for Windows, `make` may be available via Git Bash.

# Docker Desktop (optional — only needed if you ever want to build the container locally;
# Cloud Build does it in CI otherwise)
winget install Docker.DockerDesktop
```

Verify:

```powershell
terraform -v
make --version
gcloud --version
gh --version
```

---

## 1. GCP project bootstrap (Day 0, [You])

### 1.1 Create the dev project + attach billing

```powershell
# Pick a globally unique project ID. Doc default: tacitus-agon-dev
$PROJECT = "tacitus-agon-dev"
gcloud projects create $PROJECT --name="AGON dev"

# Attach a billing account (list first to find the ID)
gcloud billing accounts list
$BILLING = "<paste-your-BILLING_ACCOUNT_ID>"
gcloud billing projects link $PROJECT --billing-account=$BILLING

# Set as active
gcloud config set project $PROJECT
gcloud config set account giulio@tacitus.me
```

### 1.2 Authenticate ADC for Terraform

```powershell
gcloud auth login
gcloud auth application-default login
gcloud auth application-default set-quota-project tacitus-agon-dev
```

### 1.3 Quota uplift for Vertex AI (optional but recommended before demos)

Console → IAM & Admin → Quotas → search "Gemini 2.5 Flash" → request 60 RPM → 600 RPM.

### 1.4 Edit `.env`

`.env` is gitignored. Copy from `.env.example`:

```powershell
copy .env.example .env
# Then open .env and confirm:
#   GCP_PROJECT_ID=tacitus-agon-dev
#   GCP_REGION=us-central1
#   ENV=dev
```

---

## 2. Run bootstrap (Day 0, [Auto] but invokes gcloud)

From Git Bash or WSL:

```bash
cd /c/Users/giuli/AGON
make bootstrap
```

This enables 17 GCP APIs, creates the Terraform state bucket `${PROJECT}-terraform-state`, and grants the Cloud Build service account the IAM roles it needs.

---

## 3. Provision infrastructure (Day 2, [Auto] once Terraform files exist)

```bash
make infra-plan      # review
make infra-apply     # ~8 min
```

This creates: VPC + private subnet, Cloud SQL (Postgres 16, private IP, `db-f1-micro`), GCS buckets (documents, exports), Artifact Registry, Cloud Run service (placeholder image), Cloud Run Job, Eventarc trigger, Secret Manager entries, IAM service accounts.

After apply, `make url` returns a `https://agon-dev-xxxxx-uc.a.run.app` URL serving the placeholder hello page.

> **Status:** Terraform modules are not yet written. This is Day 2 of the build plan. The current scaffold has only the bootstrap script. The agent will write the Terraform once Day 1 (`aco-core`) is finished.

---

## 4. Connect GitHub to Cloud Build (Day 7, [You] OAuth)

```bash
make ci-connect
```

Follow the prompts. You will:

1. Install the Google Cloud Build GitHub App on the `sargonxg` account if not already installed: <https://github.com/apps/google-cloud-build>
2. Grant the app access to `sargonxg/AGON`.
3. Complete the OAuth flow in the browser when `gcloud builds connections` prints an `authUri`.

When the trigger is created, every push to `main` runs the build defined in `infra/cloudbuild.yaml` and rolls a new Cloud Run revision.

---

## 5. First deploy (Day 7, [Auto])

```bash
git push origin main
```

Watch the build:

```bash
gcloud builds list --ongoing --project=tacitus-agon-dev
gcloud builds log $(gcloud builds list --ongoing --project=tacitus-agon-dev --format='value(id)' --limit=1) --project=tacitus-agon-dev
```

When green, visit `make url`.

---

## 6. Daily loop (after Day 7)

```bash
# Edit code, push, wait ~5 min for new revision
git push origin main

# Tail logs
make logs

# Roll back if needed
make status                                                # list revisions
make rollback REVISION=agon-dev-00042-abc
```

---

## What the agent can / cannot do

| Step | Agent (Claude) | You |
|---|---|---|
| Scaffold repo files | ✓ | |
| Write Rust crates per BUILDPLAN | ✓ | |
| Write Terraform modules | ✓ | |
| Write Cloud Build / Dockerfile | ✓ | |
| Commit + push to GitHub | ✓ (when authorized) | |
| `gcloud projects create` + billing attach | | ✓ (interactive billing) |
| `gcloud auth login` (browser) | | ✓ |
| Cloud Build GitHub App install | | ✓ (browser OAuth) |
| Vertex AI quota uplift request | | ✓ (console) |
| `make bootstrap` (after auth set) | ✓ | |
| `make infra-apply` (after auth set) | ✓ | |
| Trigger a deploy via `git push` | ✓ | |
| Approve destructive ops (destroy, rollback) | | ✓ |

---

*Last updated: 2026-05-10*
