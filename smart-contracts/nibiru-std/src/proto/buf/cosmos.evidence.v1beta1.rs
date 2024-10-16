// @generated
/// Equivocation implements the Evidence interface and defines evidence of double
/// signing misbehavior.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Equivocation {
    /// height is the equivocation height.
    #[prost(int64, tag="1")]
    pub height: i64,
    /// time is the equivocation time.
    #[prost(message, optional, tag="2")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// power is the equivocation validator power.
    #[prost(int64, tag="3")]
    pub power: i64,
    /// consensus_address is the equivocation validator consensus address.
    #[prost(string, tag="4")]
    pub consensus_address: ::prost::alloc::string::String,
}
/// GenesisState defines the evidence module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// evidence defines all the evidence at genesis.
    #[prost(message, repeated, tag="1")]
    pub evidence: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// QueryEvidenceRequest is the request type for the Query/Evidence RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEvidenceRequest {
    /// evidence_hash defines the hash of the requested evidence.
    /// Deprecated: Use hash, a HEX encoded string, instead.
    #[deprecated]
    #[prost(bytes="bytes", tag="1")]
    pub evidence_hash: ::prost::bytes::Bytes,
    /// hash defines the evidence hash of the requested evidence.
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, tag="2")]
    pub hash: ::prost::alloc::string::String,
}
/// QueryEvidenceResponse is the response type for the Query/Evidence RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEvidenceResponse {
    /// evidence returns the requested evidence.
    #[prost(message, optional, tag="1")]
    pub evidence: ::core::option::Option<::prost_types::Any>,
}
/// QueryEvidenceRequest is the request type for the Query/AllEvidence RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEvidenceRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<crate::proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllEvidenceResponse is the response type for the Query/AllEvidence RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEvidenceResponse {
    /// evidence returns all evidences.
    #[prost(message, repeated, tag="1")]
    pub evidence: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<crate::proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgSubmitEvidence represents a message that supports submitting arbitrary
/// Evidence of misbehavior such as equivocation or counterfactual signing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEvidence {
    /// submitter is the signer account address of evidence.
    #[prost(string, tag="1")]
    pub submitter: ::prost::alloc::string::String,
    /// evidence defines the evidence of misbehavior.
    #[prost(message, optional, tag="2")]
    pub evidence: ::core::option::Option<::prost_types::Any>,
}
/// MsgSubmitEvidenceResponse defines the Msg/SubmitEvidence response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEvidenceResponse {
    /// hash defines the hash of the evidence.
    #[prost(bytes="bytes", tag="4")]
    pub hash: ::prost::bytes::Bytes,
}
// @@protoc_insertion_point(module)
