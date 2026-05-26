//! Merchant Invoicing & Automated Tax Calculation (Issue #333).
//!
//! Provides dynamic tax engine, automated invoice generation, accounting
//! software integration, and FIRS-formatted tax collection reports.

pub mod handlers;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;
pub mod tax_engine;

pub use models::*;
pub use repository::InvoicingRepository;
pub use routes::merchant_invoicing_routes;
pub use service::MerchantInvoicingService;
