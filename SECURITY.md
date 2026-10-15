# Security Policy

## Project status

AGON is experimental research software. No version is currently supported as a production, legal, clinical, mediation, human-resources, or autonomous decision system. Security fixes should target the default branch unless a maintainer identifies another affected revision.

## Reporting a vulnerability

Report suspected vulnerabilities privately to **hello@tacitus.me**. Do not open a public issue containing exploit details, credentials, private records, personal data, or confidential dispute material.

Please include:

- the affected commit or release;
- the component and configuration involved;
- reproduction steps or a minimal proof of concept;
- the likely impact;
- whether any real data or credential may have been exposed; and
- a safe way to contact you for follow-up.

A report will be acknowledged when reviewed. No response-time or bounty commitment is currently offered.

## Sensitive data and model providers

Do not upload confidential case files or personal data to an AGON deployment without independently reviewing authentication, logging, persistence, model-provider terms, geographic processing, retention, and deletion behavior. Use synthetic or public material for demonstrations.

Never commit API keys, service-account files, database credentials, model tokens, private source material, or generated secrets. Rotate any credential that may have entered Git history, logs, screenshots, build artifacts, or issue discussions.

## Disclosure

Allow reasonable time for investigation and remediation before public disclosure. Maintainers may request coordinated disclosure when users or deployments could remain exposed.
