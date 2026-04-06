Rating Summary

  ┌──────────┬──────────────┬───────────────┬──────────────────────┬───────────────────────────────────┐
  │ Provider │ Completeness │ Test Coverage │ Production Readiness │ Native SDK Replacement Candidate? │
  ├──────────┼──────────────┼───────────────┼──────────────────────┼───────────────────────────────────┤
  │ GCP      │     9/10     │     9/10      │         9/10         │                No                 │
  ├──────────┼──────────────┼───────────────┼──────────────────────┼───────────────────────────────────┤
  │ AWS      │     7/10     │     8/10      │         6/10         │     Yes — strongest candidate     │
  ├──────────┼──────────────┼───────────────┼──────────────────────┼───────────────────────────────────┤
  │ Azure    │     7/10     │     7/10      │         7/10         │               Maybe               │
  └──────────┴──────────────┴───────────────┴──────────────────────┴───────────────────────────────────┘

  ---
  GCP — The Gold Standard (9/10)

  What's implemented:
  - 6 credential types: Service Account, Authorized User, Metadata Server, ADC chain, Gcloud CLI, Workload Identity
  Federation (GitHub/AWS/Azure OIDC)
  - TokenProvider trait with proper async interface
  - CachedToken with RwLock, 60s expiry margin, async cache invalidation on 401
  - Quota project ID support across all credential types
  - Smart metadata server detection (TCP SYN with 1s timeout — avoids 60s hang on non-GCP)

  What's missing (minor):
  - No reachability cache for metadata server probe
  - No PKCS#8 key format support
  - No universe domain support
  - Custom HTTP clients not injectable in all credential types

  Verdict: This is production-grade. The only things missing are niche features. Not a replacement candidate — the
  custom implementation is more lightweight than google-auth-library-rust and covers all common scenarios including
  Workload Identity Federation.

  ---
  AWS — The Weakest Link (7/10)

  What's implemented:
  - Full credential chain: env vars, ~/.aws/credentials file, ECS container, EC2 IMDSv2
  - Complete SigV4 signing (validated against AWS official test vectors)
  - Robust INI parser for credentials file
  - 2-second timeout on metadata requests

  Critical gaps:
  1. No automatic credential rotation — credentials are immutable after client creation. ECS/IMDS tokens expire and are
  never refreshed
  2. No SSO credential support — aws sso login profiles are ignored
  3. No STS AssumeRole — can't use role chaining or cross-account roles
  4. No ~/.aws/config parsing — region, role_arn, source_profile not read
  5. No presigned URL generation
  6. No unsigned payload option (needed for S3 streaming)
  7. No chunked transfer SigV4
  8. retry_on_401 config exists but is unused

  Why it's the strongest replacement candidate:
  - The credential chain is the most complex of the three providers and the most incomplete
  - AWS SigV4 is a well-defined spec — aws-sigv4 crate is battle-tested and maintained by AWS
  - aws-credential-types + aws-config handle the full chain (SSO, AssumeRole, process credentials, IMDS rotation) with
  ~50 crates but zero maintenance burden
  - The lack of credential rotation is a production risk for ECS/Lambda/EC2 workloads where tokens expire every 6-12
  hours

  What native SDK replacement would look like:
  - Replace AwsCredentials + from_default_chain() with aws-config::load_defaults() +
  aws-credential-types::provider::ProvideCredentials
  - Keep the custom sigv4.rs (it's correct and lightweight) OR adopt aws-sigv4 for presigned URL support
  - Biggest win: automatic credential rotation for ECS/IMDS tokens

  ---
  Azure — Solid but Narrow (7/10)

  What's implemented:
  - 3-method credential chain: Service Principal, Managed Identity, Azure CLI
  - Dual-scope token support (ARM vs Graph) via separate HTTP methods
  - CachedToken with 5-minute buffer, proactive refresh at 55 minutes
  - Smart IMDS timeout (3s) to avoid hangs
  - Handles Azure's dual expiry formats (expires_in + expires_on)

  Gaps:
  1. No certificate-based SP auth — only client secret
  2. No Workload Identity Federation (AKS pod identity / federated credentials)
  3. No Device Code flow (browser-less auth)
  4. Graph tokens not cached in Service Principal path — fresh token per Graph call
  5. No shared token cache (OS credential store)
  6. No DefaultAzureCredential-style environment detection (e.g., App Service, Azure Functions)

  Replacement assessment:
  - azure_identity crate provides DefaultAzureCredential with ~10 credential sources
  - But it drags in the full Azure SDK ecosystem (azure_core, azure_identity, typespec_client_core) — heavy
  - The current implementation covers the 3 most common auth methods well
  - Workload Identity Federation is the biggest gap for Kubernetes workloads

  Verdict: A maybe — worth revisiting if you need AKS workload identity or certificate auth. Otherwise, the current
  implementation is lean and sufficient.

  ---
  Recommendation

  Priority order for native SDK replacement:

  1. AWS auth — Replace first. Credential rotation is a production correctness issue, not just a feature gap. The
  missing SSO/AssumeRole/config-file support limits developer ergonomics significantly.
  2. Azure auth — Consider only if you need workload identity federation for AKS or certificate-based SP auth. Current
  implementation is fine for VM/CLI/SP workflows.
  3. GCP auth — Don't replace. It's the most complete, best tested, and lightest of the three. The custom implementation
   is arguably better than pulling in the Google auth crate.
