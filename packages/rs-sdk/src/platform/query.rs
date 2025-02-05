//! Query trait representing criteria for fetching data from the platform.
//!
//! [Query] trait is used to specify individual objects as well as search criteria for fetching multiple objects from the platform.
use std::fmt::Debug;

use dapi_grpc::platform::v0::{
    self as proto, get_identity_keys_request, get_identity_keys_request::GetIdentityKeysRequestV0,
    AllKeys, GetEpochsInfoRequest, GetIdentityKeysRequest, GetProtocolVersionUpgradeStateRequest,
    KeyRequestType,
};
use dpp::{block::epoch::EpochIndex, prelude::Identifier};
use drive::query::DriveQuery;
use rs_dapi_client::transport::TransportRequest;

use crate::{error::Error, platform::document_query::DocumentQuery};

/// Default limit of epoch records returned by the platform.
pub const DEFAULT_EPOCH_QUERY_LIMIT: u32 = 100;

/// Trait implemented by objects that can be used as queries.
///
/// [Query] trait is used to specify criteria for fetching data from the platform.
/// It can be used to specify individual objects as well as search criteria for fetching multiple objects from the platform.
///
/// Some examples of queries include:
///
/// 1. [`Identifier`](crate::platform::Identifier) - fetches an object by its identifier; implemented for
/// [Identity](dpp::prelude::Identity), [DataContract](dpp::prelude::DataContract) and [Document](dpp::document::Document).
/// 2. [`DocumentQuery`] - fetches [Document](dpp::document::Document) based on search
/// conditions; see
/// [query syntax documentation](https://docs.dash.org/projects/platform/en/stable/docs/reference/query-syntax.html)
/// for more details.
///
/// ## Example
///
/// To fetch individual [Identity](dpp::prelude::Identity) object by its [Identifier](crate::platform::Identifier),
/// you just need to create it and use [Fetch](crate::platform::Fetch)
/// or [FetchMany](crate::platform::FetchMany) trait:
///
/// ```rust
/// use rs_sdk::{Sdk, platform::{Query, Identifier, Fetch, Identity}};
///
/// # const SOME_IDENTIFIER : [u8; 32] = [0; 32];
/// let mut sdk = Sdk::new_mock();
/// let query = Identifier::new(SOME_IDENTIFIER);
/// let identity = Identity::fetch(&mut sdk, query);
/// ```
///
/// As [Identifier](crate::platform::Identifier) implements [Query], the `query` variable in the code
/// above can be used as a parameter for [Fetch::fetch()](crate::platform::Fetch::fetch())
/// and [FetchMany::fetch_many()](crate::platform::FetchMany::fetch_many()) methods.
pub trait Query<T: TransportRequest>: Send + Debug + Clone {
    /// Converts the current instance into an instance of the `TransportRequest` type.
    ///
    /// This method takes ownership of the instance upon which it's called (hence `self`), and attempts to perform the conversion.
    ///
    /// # Arguments
    ///
    /// * `prove` - Whether to include proofs in the response. Only `true` is supported at the moment.
    ///
    /// # Returns
    /// On success, this method yields an instance of the `TransportRequest` type (`T`).
    /// On failure, it yields an [`Error`].
    ///
    /// # Error Handling
    /// This method propagates any errors encountered during the conversion process.
    /// These are returned as [`Error`] instances.
    fn query(self, prove: bool) -> Result<T, Error>;
}

impl<T> Query<T> for T
where
    T: TransportRequest + Sized + Send + Sync + Clone + Debug,
    T::Response: Send + Sync + Debug,
{
    fn query(self, prove: bool) -> Result<T, Error> {
        if !prove {
            unimplemented!("queries without proofs are not supported yet");
        }
        Ok(self)
    }
}

impl Query<proto::GetDataContractRequest> for Identifier {
    fn query(self, prove: bool) -> Result<proto::GetDataContractRequest, Error> {
        if !prove {
            unimplemented!("queries without proofs are not supported yet");
        }
        let id = self.to_vec();
        Ok(proto::GetDataContractRequest {
            version: Some(proto::get_data_contract_request::Version::V0(
                proto::get_data_contract_request::GetDataContractRequestV0 { id, prove: true },
            )),
        })
    }
}

impl Query<proto::GetIdentityKeysRequest> for Identifier {
    /// Get all keys for an identity with provided identifier.
    fn query(self, prove: bool) -> Result<proto::GetIdentityKeysRequest, Error> {
        if !prove {
            unimplemented!("queries without proofs are not supported yet");
        }
        let identity_id = self.to_vec();
        Ok(GetIdentityKeysRequest {
            version: Some(get_identity_keys_request::Version::V0(
                GetIdentityKeysRequestV0 {
                    identity_id,
                    prove,
                    limit: None,
                    offset: None,
                    request_type: Some(KeyRequestType {
                        request: Some(proto::key_request_type::Request::AllKeys(AllKeys {})),
                    }),
                },
            )),
        })
    }
}

impl<'a> Query<DocumentQuery> for DriveQuery<'a> {
    fn query(self, prove: bool) -> Result<DocumentQuery, Error> {
        if !prove {
            unimplemented!("queries without proofs are not supported yet");
        }
        let q: DocumentQuery = (&self).into();
        Ok(q)
    }
}

/// Wrapper around query that allows to specify limit and offset.
///
/// A query that can be used specify limit and offset when fetching multiple objects from the platform
/// using [`FetchMany`](crate::platform::FetchMany) trait.
///
/// ## Example
///
/// ```rust
/// use rs_sdk::{Sdk, platform::{Query, LimitQuery, Identifier, FetchMany, Identity}};
/// use drive_proof_verifier::types::ExtendedEpochInfos;
/// use dpp::block::extended_epoch_info::ExtendedEpochInfo;
///
/// # const SOME_IDENTIFIER : [u8; 32] = [0; 32];
/// let mut sdk = Sdk::new_mock();
/// let query = LimitQuery {
///    query: 1,
///    limit: Some(10),
///    offset: Some(5),
/// };
/// let epoch = ExtendedEpochInfo::fetch_many(&mut sdk, query);
/// ```
#[derive(Debug, Clone)]
pub struct LimitQuery<Q> {
    /// Actual query to execute
    pub query: Q,
    /// Max number of records returned
    pub limit: Option<u32>,
    /// Start offset. Will return records starting from this offset
    /// up to `offset+limit`.
    pub offset: Option<u32>,
}
impl<Q> From<Q> for LimitQuery<Q> {
    fn from(query: Q) -> Self {
        Self {
            query,
            limit: None,
            offset: None,
        }
    }
}

impl Query<GetEpochsInfoRequest> for LimitQuery<EpochIndex> {
    fn query(self, prove: bool) -> Result<GetEpochsInfoRequest, Error> {
        if !prove {
            unimplemented!("queries without proofs are not supported yet");
        }

        if self.offset.is_some() {
            unimplemented!("offset is not supported for epoch queries");
        }

        Ok(GetEpochsInfoRequest {
            version: Some(proto::get_epochs_info_request::Version::V0(
                proto::get_epochs_info_request::GetEpochsInfoRequestV0 {
                    prove,
                    start_epoch: Some(self.query.into()),
                    count: self.limit.unwrap_or(DEFAULT_EPOCH_QUERY_LIMIT),
                    ascending: true,
                },
            )),
        })
    }
}

impl Query<GetEpochsInfoRequest> for EpochIndex {
    fn query(self, prove: bool) -> Result<GetEpochsInfoRequest, Error> {
        LimitQuery::from(self).query(prove)
    }
}

impl Query<GetProtocolVersionUpgradeStateRequest> for () {
    fn query(self, prove: bool) -> Result<GetProtocolVersionUpgradeStateRequest, Error> {
        if !prove {
            unimplemented!("queries without proofs are not supported yet");
        }

        Ok(proto::get_protocol_version_upgrade_state_request::GetProtocolVersionUpgradeStateRequestV0 {prove}.into())
    }
}
