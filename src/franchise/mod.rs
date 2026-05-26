//! Multi-Store & Franchise Management (Issue #335).
//!
//! Provides a parent-child account hierarchy: Organization → Region → Branch,
//! with RBAC, consolidated settlement, and cross-store analytics.

pub mod handlers;
pub mod models;
pub mod rbac;
pub mod repository;
pub mod routes;
pub mod service;

pub use models::*;
pub use repository::FranchiseRepository;
pub use routes::franchise_routes;
pub use service::FranchiseService;
