//! Integration test entry point for azure-lite.
//!
//! Each service has its own module under `tests/integration/`.
//! Run with: cargo test -p azure-lite --test integration -- --ignored --test-threads=1 --nocapture

mod integration {
    pub mod acr;
    pub mod aks;
    pub mod auth;
    pub mod compute;
    pub mod cosmosdb;
    pub mod cost;
    pub mod dns;
    pub mod functions;
    pub mod graph;
    pub mod identity;
    pub mod keyvault;
    pub mod loganalytics;
    pub mod monitor;
    pub mod networking;
    pub mod networking_lb;
    pub mod rbac;
    pub mod redis;
    pub mod resource_graph;
    pub mod security;
    pub mod sql;
    pub mod storage;
    pub mod subscriptions;
}
