//! Merchant CRM & Customer Insights (Issue #334)
//!
//! Provides merchants with customer profiling, segmentation, purchasing pattern
//! analytics, and privacy-first data export capabilities.

pub mod encryption;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;

pub use models::*;
pub use repository::CustomerProfileRepository;
pub use routes::merchant_crm_routes;
pub use service::MerchantCrmService;
