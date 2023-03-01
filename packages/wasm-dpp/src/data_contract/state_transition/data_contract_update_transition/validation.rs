use std::{collections::BTreeMap, sync::Arc};

use dpp::{
    data_contract::state_transition::data_contract_update_transition::validation::{
        basic::{
            validate_indices_are_backward_compatible as dpp_validate_indices_are_backward_compatible,
            DataContractUpdateTransitionBasicValidator,
        },
        state::validate_data_contract_update_transition_state::validate_data_contract_update_transition_state as dpp_validate_data_contract_update_transition_state,
    },
    version::ProtocolVersionValidator,
};
use wasm_bindgen::prelude::*;

use crate::{
    data_contract::state_transition::data_contract_update_transition::DataContractUpdateTransitionParameters,
    errors::{from_dpp_err, protocol_error::from_protocol_error},
    state_repository::{ExternalStateRepositoryLike, ExternalStateRepositoryLikeWrapper},
    validation::ValidationResultWasm,
    DataContractUpdateTransitionWasm, StateTransitionExecutionContextWasm,
};

#[wasm_bindgen(js_name=validateDataContractUpdateTransitionState)]
pub async fn validate_data_contract_update_transition_state(
    state_repository: ExternalStateRepositoryLike,
    state_transition: DataContractUpdateTransitionWasm,
) -> Result<ValidationResultWasm, JsValue> {
    let wrapped_state_repository = ExternalStateRepositoryLikeWrapper::new(state_repository);
    let result = dpp_validate_data_contract_update_transition_state(
        &wrapped_state_repository,
        &state_transition.into(),
    )
    .await
    .map_err(from_dpp_err)?;

    Ok(result.map(|_| JsValue::undefined()).into())
}

#[wasm_bindgen(js_name=validateIndicesAreBackwardCompatible)]
pub fn validate_indices_are_backward_compatible(
    old_documents_schema: JsValue,
    new_documents_schema: JsValue,
) -> Result<ValidationResultWasm, JsValue> {
    let old_documents = serde_wasm_bindgen::from_value::<BTreeMap<String, serde_json::Value>>(
        old_documents_schema,
    )?;
    let new_documents = serde_wasm_bindgen::from_value::<BTreeMap<String, serde_json::Value>>(
        new_documents_schema,
    )?;

    let result =
        dpp_validate_indices_are_backward_compatible(old_documents.iter(), new_documents.iter())
            .map_err(from_protocol_error)?;

    Ok(result.map(|_| JsValue::undefined()).into())
}

#[wasm_bindgen(js_name=validateDataContractUpdateTransitionBasic)]
pub async fn validate_data_contract_update_transition_basic(
    state_repository: ExternalStateRepositoryLike,
    raw_parameters: JsValue,
    execution_context: &StateTransitionExecutionContextWasm,
) -> Result<ValidationResultWasm, JsError> {
    let parameters: DataContractUpdateTransitionParameters =
        serde_wasm_bindgen::from_value(raw_parameters)?;

    let validator: DataContractUpdateTransitionBasicValidator<ExternalStateRepositoryLikeWrapper> =
        DataContractUpdateTransitionBasicValidator::new(
            Arc::new(ExternalStateRepositoryLikeWrapper::new(state_repository)),
            Arc::new(ProtocolVersionValidator::default()),
        )?;

    let validation_result = validator
        .validate(
            &serde_json::to_value(&parameters)?,
            &execution_context.into(),
        )
        .await?;

    Ok(validation_result.map(|_| JsValue::undefined()).into())
}
