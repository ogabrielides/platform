//! Configuration helpers for mocking of rs-sdk.
//!
//! This module contains [Config] struct that can be used to configure rs-sdk.
//! It's mainly used for testing.

use dpp::prelude::Identifier;
use rs_dapi_client::AddressList;
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};

/// Existing document ID
///
// TODO: this is copy-paste from drive-abci `packages/rs-sdk/tests/fetch/main.rs` where it's private,
// consider defining it in `data-contracts` crate
const DPNS_DASH_TLD_DOCUMENT_ID: [u8; 32] = [
    215, 242, 197, 63, 70, 169, 23, 171, 110, 91, 57, 162, 215, 188, 38, 11, 100, 146, 137, 69, 55,
    68, 209, 224, 212, 242, 106, 141, 142, 255, 55, 207,
];

#[derive(Debug, Deserialize)]
/// Configuration for rs-sdk.
///
/// Content of this configuration is loaded from environment variables or `${CARGO_MANIFEST_DIR}/.env` file
/// when the [Config::new()] is called.
/// Variable names in the enviroment and `.env` file must be prefixed with [RS_SDK_](Config::CONFIG_PREFIX)
/// and written as SCREAMING_SNAKE_CASE (e.g. `RS_SDK_PLATFORM_HOST`).
pub struct Config {
    /// Hostname of the Dash Platform node to connect to
    #[serde(default)]
    pub platform_host: String,
    /// Port of the Dash Platform node grpc interface
    #[serde(default)]
    pub platform_port: u16,
    /// Port of the Dash Core RPC interface running on the Dash Platform node
    #[serde(default)]
    pub core_port: u16,
    /// Username for Dash Core RPC interface
    #[serde(default)]
    pub core_user: String,
    /// Password for Dash Core RPC interface
    #[serde(default)]
    pub core_password: String,

    /// Directory where all generated test vectors will be saved.
    ///
    /// See [SdkBuilder::with_dump_dir()](crate::SdkBuilder::with_dump_dir()) for more details.
    #[serde(default = "Config::default_dump_dir")]
    pub dump_dir: PathBuf,

    // IDs of some objects generated by the testnet
    /// ID of existing identity.
    ///
    /// Format: Base58
    #[serde(default = "Config::default_identity_id")]
    pub existing_identity_id: Identifier,
    /// ID of existing data contract.
    ///
    /// Format: Base58
    #[serde(default = "Config::default_data_contract_id")]
    pub existing_data_contract_id: Identifier,
    /// Name of document type defined for [`existing_data_contract_id`](Config::existing_data_contract_id).
    #[serde(default = "Config::default_document_type_name")]
    pub existing_document_type_name: String,
    /// ID of document of the type [`existing_document_type_name`](Config::existing_document_type_name)
    /// in [`existing_data_contract_id`](Config::existing_data_contract_id).
    #[serde(default = "Config::default_document_id")]
    pub existing_document_id: Identifier,
}

impl Config {
    /// Prefix of configuration options in the environment variables and `.env` file.
    pub const CONFIG_PREFIX: &str = "RS_SDK_";
    /// Load configuration from operating system environment variables and `.env` file.
    ///
    /// Create new [Config] with data from environment variables and `${CARGO_MANIFEST_DIR}/tests/.env` file.
    /// Variable names in the environment and `.env` file must be converted to SCREAMING_SNAKE_CASE and
    /// prefixed with [RS_SDK_](Config::CONFIG_PREFIX).
    pub fn new() -> Self {
        // load config from .env file, ignore errors

        let path: String = env!("CARGO_MANIFEST_DIR").to_owned() + "/tests/.env";
        if let Err(err) = dotenvy::from_path(&path) {
            tracing::warn!(path, ?err, "failed to load config file");
        }

        let config: Self = envy::prefixed(Self::CONFIG_PREFIX)
            .from_env()
            .expect("configuration error");

        if config.is_empty() {
            tracing::warn!(path, ?config, "some config fields are empty");
            #[cfg(not(feature = "offline-testing"))]
            panic!("invalid configuration")
        }

        config
    }

    /// Check if credentials of the config are empty.
    ///
    /// Checks if fields [platform_host](Config::platform_host), [platform_port](Config::platform_port),
    /// [core_port](Config::core_port), [core_user](Config::core_user) and [core_password](Config::core_password)
    /// are not empty.
    ///
    /// Other fields are ignored.
    pub fn is_empty(&self) -> bool {
        self.core_user.is_empty()
            || self.core_password.is_empty()
            || self.platform_host.is_empty()
            || self.platform_port == 0
            || self.core_port == 0
    }

    #[allow(unused)]
    /// Create list of Platform addresses from the configuration
    pub fn address_list(&self) -> AddressList {
        let address: String = format!("http://{}:{}", self.platform_host, self.platform_port);

        AddressList::from_iter(vec![http::Uri::from_str(&address).expect("valid uri")])
    }

    /// Create new SDK instance
    ///
    /// Depending on the feature flags, it will connect to the configured platform node or mock API.
    ///
    /// ## Feature flags
    ///
    /// * `offline-testing` is not set - connect to the platform and generate
    /// new test vectors during execution
    /// * `offline-testing` is set - use mock implementation and
    /// load existing test vectors from disk
    pub async fn setup_api(&self) -> rs_sdk::Sdk {
        // offline testing takes precedence over network testing
        #[cfg(all(feature = "network-testing", not(feature = "offline-testing")))]
        let sdk = {
            // Dump all traffic to disk
            let builder = rs_sdk::SdkBuilder::new(self.address_list()).with_core(
                &self.platform_host,
                self.core_port,
                &self.core_user,
                &self.core_password,
            );

            #[cfg(feature = "generate-test-vectors")]
            let builder = builder.with_dump_dir(&self.dump_dir);

            builder.build().expect("cannot initialize api")
        };

        // offline testing takes precedence over network testing
        #[cfg(feature = "offline-testing")]
        let sdk = {
            let mut mock_sdk = rs_sdk::SdkBuilder::new_mock()
                .build()
                .expect("initialize api");

            mock_sdk
                .mock()
                .quorum_info_dir(&self.dump_dir)
                .load_expectations(&self.dump_dir)
                .await
                .expect("load expectations");

            mock_sdk
        };

        sdk
    }

    fn default_identity_id() -> Identifier {
        data_contracts::SystemDataContract::DPNS
            .source()
            .expect("data contract source")
            .owner_id_bytes
            .into()
    }

    fn default_data_contract_id() -> Identifier {
        data_contracts::SystemDataContract::DPNS.id()
    }

    fn default_document_type_name() -> String {
        "domain".to_string()
    }
    fn default_document_id() -> Identifier {
        DPNS_DASH_TLD_DOCUMENT_ID.into()
    }

    fn default_dump_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("vectors")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
