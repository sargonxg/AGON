.PHONY: help bootstrap infra-plan infra-apply infra-destroy ci-connect deploy logs url url-raw status rollback test-local test-cloud

include .env
export

help:
	@echo "make bootstrap        - one-time GCP project setup"
	@echo "make infra-plan       - terraform plan"
	@echo "make infra-apply      - terraform apply"
	@echo "make infra-destroy    - terraform destroy"
	@echo "make ci-connect       - connect GitHub repo to Cloud Build"
	@echo "make deploy           - push to main (CI builds and deploys)"
	@echo "make logs             - tail Cloud Run logs"
	@echo "make url              - print deployed Cloud Run URL"
	@echo "make status           - show Cloud Run revisions"
	@echo "make rollback REVISION=<rev> - roll back to a previous revision"
	@echo "make test-local       - cargo test against testcontainers Postgres"
	@echo "make test-cloud       - integration tests against deployed dev env"

bootstrap:
	bash infra/bootstrap.sh

infra-plan:
	cd infra/terraform/envs/$(ENV) && terraform init && terraform plan

infra-apply:
	cd infra/terraform/envs/$(ENV) && terraform apply

infra-destroy:
	cd infra/terraform/envs/$(ENV) && terraform destroy

ci-connect:
	bash infra/connect-github.sh

deploy:
	git push origin main

logs:
	gcloud logging tail "resource.type=cloud_run_revision AND resource.labels.service_name=agon-$(ENV)" --project=$(GCP_PROJECT_ID)

url:
	@gcloud run services describe agon-$(ENV) --region=$(GCP_REGION) --project=$(GCP_PROJECT_ID) --format='value(status.url)'

url-raw:
	@gcloud run services describe agon-$(ENV) --region=$(GCP_REGION) --project=$(GCP_PROJECT_ID) --format='value(status.url)' | tr -d '\n'

status:
	gcloud run revisions list --service=agon-$(ENV) --region=$(GCP_REGION) --project=$(GCP_PROJECT_ID)

rollback:
	gcloud run services update-traffic agon-$(ENV) --region=$(GCP_REGION) --project=$(GCP_PROJECT_ID) --to-revisions=$(REVISION)=100

test-local:
	cargo test --workspace --all-features

test-cloud:
	cargo test --workspace --all-features --features live-vertex -- --ignored
