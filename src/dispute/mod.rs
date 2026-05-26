//! Merchant Dispute Resolution & Clawback Management (Issue #337).
//!
//! Provides a structured workflow for customers and merchants to resolve
//! payment disputes without reverting to manual banking investigations.
//! Uses blockchain's immutable ledger for Proof of Delivery and supports
//! mediated Clawback / Partial Refund processes.

pub mod handlers;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;

pub use models::*;
pub use repository::DisputeRepository;
pub use routes::dispute_routes;
pub use service::DisputeService;
