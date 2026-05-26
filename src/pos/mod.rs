// ============================================================================
// POS (Point of Sale) QR Payment Protocol — Physical Retail Integration
// ============================================================================
// Implements SEP-7 compliant QR code generation for brick-and-mortar merchants
// to accept cNGN payments via Stellar-enabled wallets.
//
// Architecture:
// - Dynamic QR generation with embedded payment intent (amount, memo, destination)
// - Real-time WebSocket listener for instant payment confirmation
// - Legacy POS bridge for integration with existing retail software
// - Offline-to-online validation with proof-of-payment screens
//
// Performance targets:
// - QR generation: <300ms
// - Payment confirmation: <3s from customer signature
// - Handles overpayment/underpayment detection
// ============================================================================

pub mod handlers;
pub mod legacy_bridge;
pub mod lobby_service;
pub mod models;
pub mod payment_intent;
pub mod proof_of_payment;
pub mod qr_generator;
pub mod routes;
pub mod validation;
pub mod websocket;

pub use legacy_bridge::LegacyBridge;
pub use lobby_service::LobbyService;
pub use payment_intent::PaymentIntent;
pub use proof_of_payment::ProofOfPayment;
pub use qr_generator::QrGenerator;
