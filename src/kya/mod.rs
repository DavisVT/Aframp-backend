// KYA (Know Your Agent) - Decentralized Agent Identity & Reputation System
//
// This module implements a sovereign identity and reputation framework for autonomous agents,
// enabling trustless collaboration through:
// - DID-based identity registry
// - On-chain reputation & attestations
// - Zero-knowledge competence proofs
// - Cross-platform reputation portability
// - Sybil-resistant feedback mechanisms

pub mod attestation;
pub mod error;
pub mod identity;
pub mod models;
pub mod registry;
pub mod reputation;
pub mod routes;
pub mod scoring;
pub mod zkp;

pub use attestation::{Attestation, AttestationVerifier};
pub use error::KYAError;
pub use identity::{AgentIdentity, IdentityRegistry};
pub use models::*;
pub use registry::KYARegistry;
pub use reputation::{FeedbackAuthorization, ReputationManager};
pub use routes::kya_routes;
pub use scoring::{DomainScore, ModularScoring};
pub use zkp::{CompetenceProof, ZKProofVerifier};
