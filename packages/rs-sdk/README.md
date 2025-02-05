# Dash Platform Rust SDK

This is the official Rust SDK for the Dash Platform. Dash Platform is a Layer 2 cryptocurrency technology that builds upon the Dash layer 1 network. This SDK provides an abstraction layer to simplify usage of the Dash Platform along with data models based on the Dash Platform Protocol (DPP), a CRUD interface, and bindings for other technologies such as C.

See Rust documentation of this crate for more details.

## Examples

You can find quick start example in `examples/` folder. Examples must be configured by setting constants.

You can also inspect tests in `tests/` folder for more detailed examples.

## Tests

This section provides instructions on how to test the RS-SDK for Dash Platform. The tests can be run in two modes: **offline** (without connectivity to the Dash Platform) and **network** (with connectivity to the Dash Platform). **Offline** mode is the default one.

If both **network** and **offline** testing is enabled, **offline testing** takes precedence.

## Network Testing

Network testing requires connectivity to the Dash Platform and Dash Core.

Follow these steps to conduct network testing:

1. Configure platform address and credentials in `packages/rs-sdk/tests/.env`.
   Note that the `.env` file might already be configured during  project setup (`yarn setup`).
2. Run the test without default features, but with `network-testing` feature enabled.

```bash
cd packages/rs-sdk
cargo test -p rs-sdk --no-default-features --features network-testing
```

## Offline Testing

Offline testing uses the vectors generated using `packages/rs-sdk/scripts/generate_test_vectors.sh` script.
These vectors must be saved in `packages/rs-sdk/tests/vectors`.

### Generating test vectors

To generate test vectors for offline testing:

1. Configure platform address and credentials in `packages/rs-sdk/tests/.env`.
   Note that the `.env` file might already be configured during project setup (`yarn setup`).
2. Run  `packages/rs-sdk/scripts/generate_test_vectors.sh` script.

### Running tests in offline mode

Run the offline test using the following command:

```bash
cargo test -p rs-sdk
```

## Implementing Fetch and FetchAny on new objects

How to implement `Fetch` and `FetchAny` trait on new object types (`Object`).
It's basically copy-paste and tewaking of existing implementation for another object type.

Definitions:

1. `Request` - gRPC request type, as generated in `packages/dapi-grpc/protos/platform/v0/platform.proto`.
2. `Response` - gRPC response  type, as generated in `packages/dapi-grpc/protos/platform/v0/platform.proto`.
3. `Object` - object type that should be returned by rs-sdk, most likely defined in `dpp` crate.
   In some cases, it can be defined in `packages/rs-drive-proof-verifier/src/types.rs`.

Checklist:

1. Ensure protobuf messages are defined in `packages/dapi-grpc/protos/platform/v0/platform.proto` and generated
   correctly in `packages/dapi-grpc/src/platform/proto/org.dash.platform.dapi.v0.rs`.
2. In `packages/dapi-grpc/build.rs`, add `Request` to `VERSIONED_REQUESTS` and response `Response` to `VERSIONED_RESPONSES`.
   This should add derive of `VersionedGrpcMessage` (and some more) in `org.dash.platform.dapi.v0.rs`.
3. Link request and response type to dapi-client by adding appropriate invocation of `impl_transport_request_grpc!` macro
in `packages/rs-dapi-client/src/transport/grpc.rs`.
4. If needed, implement new type in `packages/rs-drive-proof-verifier/src/types.rs` to hide complexity of data structures
   used internally.

   If you intend to implement `FetchMany`, you should define type returned by `fetch_many()` using `Collection`
   that will store collection of  returned objects, indexd by some key.
5. Implement `FromProof` trait for the `Object` (or type defined in `types.rs`) in `packages/rs-drive-proof-verifier/src/proof.rs`.
6. Implement `Query` trait for the `Request` in `packages/rs-sdk/src/platform/query.rs`.
7. Implement `Fetch\<Request\>` trait for the `Object` (or type defined in `types.rs`) in `packages/rs-sdk/src/platform/fetch.rs`.
8. Implement `FetchMany\<Request\>` trait for the `Object` (or type defined in `types.rs`) in `packages/rs-sdk/src/platform/fetch_many.rs`.
9. Add `mod ...;` clause to `packages/rs-sdk/tests/fetch/main.rs`
10. Implement unit tests in `packages/rs-sdk/tests/fetch/*object*.rs`
11. Start local devnet with `yarn reset ; yarn setup && yarn start`
12. Generate test vectors with script `packages/rs-sdk/scripts/generate_test_vectors.sh`
