// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: platform.proto

// This CPP symbol can be defined to use imports that match up to the framework
// imports needed when using CocoaPods.
#if !defined(GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS)
 #define GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS 0
#endif

#if GPB_USE_PROTOBUF_FRAMEWORK_IMPORTS
 #import <Protobuf/GPBProtocolBuffers.h>
#else
 #import "GPBProtocolBuffers.h"
#endif

#if GOOGLE_PROTOBUF_OBJC_VERSION < 30004
#error This file was generated by a newer version of protoc which is incompatible with your Protocol Buffer library sources.
#endif
#if 30004 < GOOGLE_PROTOBUF_OBJC_MIN_SUPPORTED_VERSION
#error This file was generated by an older version of protoc which is incompatible with your Protocol Buffer library sources.
#endif

// @@protoc_insertion_point(imports)

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

CF_EXTERN_C_BEGIN

@class ConsensusParamsBlock;
@class ConsensusParamsEvidence;
@class Proof;
@class ResponseMetadata;
@class StateTransitionBroadcastError;

NS_ASSUME_NONNULL_BEGIN

#pragma mark - PlatformRoot

/**
 * Exposes the extension registry for this file.
 *
 * The base class provides:
 * @code
 *   + (GPBExtensionRegistry *)extensionRegistry;
 * @endcode
 * which is a @c GPBExtensionRegistry that includes all the extensions defined by
 * this file and all files that it depends on.
 **/
GPB_FINAL @interface PlatformRoot : GPBRootObject
@end

#pragma mark - Proof

typedef GPB_ENUM(Proof_FieldNumber) {
  Proof_FieldNumber_MerkleProof = 1,
  Proof_FieldNumber_QuorumHash = 2,
  Proof_FieldNumber_Signature = 3,
  Proof_FieldNumber_Round = 4,
};

GPB_FINAL @interface Proof : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSData *merkleProof;

@property(nonatomic, readwrite, copy, null_resettable) NSData *quorumHash;

@property(nonatomic, readwrite, copy, null_resettable) NSData *signature;

@property(nonatomic, readwrite) uint32_t round;

@end

#pragma mark - ResponseMetadata

typedef GPB_ENUM(ResponseMetadata_FieldNumber) {
  ResponseMetadata_FieldNumber_Height = 1,
  ResponseMetadata_FieldNumber_CoreChainLockedHeight = 2,
  ResponseMetadata_FieldNumber_TimeMs = 3,
  ResponseMetadata_FieldNumber_ProtocolVersion = 4,
};

GPB_FINAL @interface ResponseMetadata : GPBMessage

@property(nonatomic, readwrite) int64_t height;

@property(nonatomic, readwrite) uint32_t coreChainLockedHeight;

@property(nonatomic, readwrite) uint64_t timeMs;

@property(nonatomic, readwrite) uint32_t protocolVersion;

@end

#pragma mark - StateTransitionBroadcastError

typedef GPB_ENUM(StateTransitionBroadcastError_FieldNumber) {
  StateTransitionBroadcastError_FieldNumber_Code = 1,
  StateTransitionBroadcastError_FieldNumber_Message = 2,
  StateTransitionBroadcastError_FieldNumber_Data_p = 3,
};

GPB_FINAL @interface StateTransitionBroadcastError : GPBMessage

@property(nonatomic, readwrite) uint32_t code;

@property(nonatomic, readwrite, copy, null_resettable) NSString *message;

@property(nonatomic, readwrite, copy, null_resettable) NSData *data_p;

@end

#pragma mark - BroadcastStateTransitionRequest

typedef GPB_ENUM(BroadcastStateTransitionRequest_FieldNumber) {
  BroadcastStateTransitionRequest_FieldNumber_StateTransition = 1,
};

GPB_FINAL @interface BroadcastStateTransitionRequest : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSData *stateTransition;

@end

#pragma mark - BroadcastStateTransitionResponse

GPB_FINAL @interface BroadcastStateTransitionResponse : GPBMessage

@end

#pragma mark - GetIdentityRequest

typedef GPB_ENUM(GetIdentityRequest_FieldNumber) {
  GetIdentityRequest_FieldNumber_Id_p = 1,
  GetIdentityRequest_FieldNumber_Prove = 2,
};

GPB_FINAL @interface GetIdentityRequest : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSData *id_p;

@property(nonatomic, readwrite) BOOL prove;

@end

#pragma mark - GetIdentityResponse

typedef GPB_ENUM(GetIdentityResponse_FieldNumber) {
  GetIdentityResponse_FieldNumber_Identity = 1,
  GetIdentityResponse_FieldNumber_Proof = 2,
  GetIdentityResponse_FieldNumber_Metadata = 3,
};

GPB_FINAL @interface GetIdentityResponse : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSData *identity;

@property(nonatomic, readwrite, strong, null_resettable) Proof *proof;
/** Test to see if @c proof has been set. */
@property(nonatomic, readwrite) BOOL hasProof;

@property(nonatomic, readwrite, strong, null_resettable) ResponseMetadata *metadata;
/** Test to see if @c metadata has been set. */
@property(nonatomic, readwrite) BOOL hasMetadata;

@end

#pragma mark - GetDataContractRequest

typedef GPB_ENUM(GetDataContractRequest_FieldNumber) {
  GetDataContractRequest_FieldNumber_Id_p = 1,
  GetDataContractRequest_FieldNumber_Prove = 2,
};

GPB_FINAL @interface GetDataContractRequest : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSData *id_p;

@property(nonatomic, readwrite) BOOL prove;

@end

#pragma mark - GetDataContractResponse

typedef GPB_ENUM(GetDataContractResponse_FieldNumber) {
  GetDataContractResponse_FieldNumber_DataContract = 1,
  GetDataContractResponse_FieldNumber_Proof = 2,
  GetDataContractResponse_FieldNumber_Metadata = 3,
};

GPB_FINAL @interface GetDataContractResponse : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSData *dataContract;

@property(nonatomic, readwrite, strong, null_resettable) Proof *proof;
/** Test to see if @c proof has been set. */
@property(nonatomic, readwrite) BOOL hasProof;

@property(nonatomic, readwrite, strong, null_resettable) ResponseMetadata *metadata;
/** Test to see if @c metadata has been set. */
@property(nonatomic, readwrite) BOOL hasMetadata;

@end

#pragma mark - GetDocumentsRequest

typedef GPB_ENUM(GetDocumentsRequest_FieldNumber) {
  GetDocumentsRequest_FieldNumber_DataContractId = 1,
  GetDocumentsRequest_FieldNumber_DocumentType = 2,
  GetDocumentsRequest_FieldNumber_Where = 3,
  GetDocumentsRequest_FieldNumber_OrderBy = 4,
  GetDocumentsRequest_FieldNumber_Limit = 5,
  GetDocumentsRequest_FieldNumber_StartAfter = 6,
  GetDocumentsRequest_FieldNumber_StartAt = 7,
  GetDocumentsRequest_FieldNumber_Prove = 8,
};

typedef GPB_ENUM(GetDocumentsRequest_Start_OneOfCase) {
  GetDocumentsRequest_Start_OneOfCase_GPBUnsetOneOfCase = 0,
  GetDocumentsRequest_Start_OneOfCase_StartAfter = 6,
  GetDocumentsRequest_Start_OneOfCase_StartAt = 7,
};

GPB_FINAL @interface GetDocumentsRequest : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSData *dataContractId;

@property(nonatomic, readwrite, copy, null_resettable) NSString *documentType;

@property(nonatomic, readwrite, copy, null_resettable) NSData *where;

@property(nonatomic, readwrite, copy, null_resettable) NSData *orderBy;

@property(nonatomic, readwrite) uint32_t limit;

@property(nonatomic, readonly) GetDocumentsRequest_Start_OneOfCase startOneOfCase;

@property(nonatomic, readwrite, copy, null_resettable) NSData *startAfter;

@property(nonatomic, readwrite, copy, null_resettable) NSData *startAt;

@property(nonatomic, readwrite) BOOL prove;

@end

/**
 * Clears whatever value was set for the oneof 'start'.
 **/
void GetDocumentsRequest_ClearStartOneOfCase(GetDocumentsRequest *message);

#pragma mark - GetDocumentsResponse

typedef GPB_ENUM(GetDocumentsResponse_FieldNumber) {
  GetDocumentsResponse_FieldNumber_DocumentsArray = 1,
  GetDocumentsResponse_FieldNumber_Proof = 2,
  GetDocumentsResponse_FieldNumber_Metadata = 3,
};

GPB_FINAL @interface GetDocumentsResponse : GPBMessage

@property(nonatomic, readwrite, strong, null_resettable) NSMutableArray<NSData*> *documentsArray;
/** The number of items in @c documentsArray without causing the array to be created. */
@property(nonatomic, readonly) NSUInteger documentsArray_Count;

@property(nonatomic, readwrite, strong, null_resettable) Proof *proof;
/** Test to see if @c proof has been set. */
@property(nonatomic, readwrite) BOOL hasProof;

@property(nonatomic, readwrite, strong, null_resettable) ResponseMetadata *metadata;
/** Test to see if @c metadata has been set. */
@property(nonatomic, readwrite) BOOL hasMetadata;

@end

#pragma mark - GetIdentitiesByPublicKeyHashesRequest

typedef GPB_ENUM(GetIdentitiesByPublicKeyHashesRequest_FieldNumber) {
  GetIdentitiesByPublicKeyHashesRequest_FieldNumber_PublicKeyHashesArray = 1,
  GetIdentitiesByPublicKeyHashesRequest_FieldNumber_Prove = 2,
};

GPB_FINAL @interface GetIdentitiesByPublicKeyHashesRequest : GPBMessage

@property(nonatomic, readwrite, strong, null_resettable) NSMutableArray<NSData*> *publicKeyHashesArray;
/** The number of items in @c publicKeyHashesArray without causing the array to be created. */
@property(nonatomic, readonly) NSUInteger publicKeyHashesArray_Count;

@property(nonatomic, readwrite) BOOL prove;

@end

#pragma mark - GetIdentitiesByPublicKeyHashesResponse

typedef GPB_ENUM(GetIdentitiesByPublicKeyHashesResponse_FieldNumber) {
  GetIdentitiesByPublicKeyHashesResponse_FieldNumber_IdentitiesArray = 1,
  GetIdentitiesByPublicKeyHashesResponse_FieldNumber_Proof = 2,
  GetIdentitiesByPublicKeyHashesResponse_FieldNumber_Metadata = 3,
};

GPB_FINAL @interface GetIdentitiesByPublicKeyHashesResponse : GPBMessage

@property(nonatomic, readwrite, strong, null_resettable) NSMutableArray<NSData*> *identitiesArray;
/** The number of items in @c identitiesArray without causing the array to be created. */
@property(nonatomic, readonly) NSUInteger identitiesArray_Count;

@property(nonatomic, readwrite, strong, null_resettable) Proof *proof;
/** Test to see if @c proof has been set. */
@property(nonatomic, readwrite) BOOL hasProof;

@property(nonatomic, readwrite, strong, null_resettable) ResponseMetadata *metadata;
/** Test to see if @c metadata has been set. */
@property(nonatomic, readwrite) BOOL hasMetadata;

@end

#pragma mark - WaitForStateTransitionResultRequest

typedef GPB_ENUM(WaitForStateTransitionResultRequest_FieldNumber) {
  WaitForStateTransitionResultRequest_FieldNumber_StateTransitionHash = 1,
  WaitForStateTransitionResultRequest_FieldNumber_Prove = 2,
};

GPB_FINAL @interface WaitForStateTransitionResultRequest : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSData *stateTransitionHash;

@property(nonatomic, readwrite) BOOL prove;

@end

#pragma mark - WaitForStateTransitionResultResponse

typedef GPB_ENUM(WaitForStateTransitionResultResponse_FieldNumber) {
  WaitForStateTransitionResultResponse_FieldNumber_Error = 1,
  WaitForStateTransitionResultResponse_FieldNumber_Proof = 2,
  WaitForStateTransitionResultResponse_FieldNumber_Metadata = 3,
};

typedef GPB_ENUM(WaitForStateTransitionResultResponse_Responses_OneOfCase) {
  WaitForStateTransitionResultResponse_Responses_OneOfCase_GPBUnsetOneOfCase = 0,
  WaitForStateTransitionResultResponse_Responses_OneOfCase_Error = 1,
  WaitForStateTransitionResultResponse_Responses_OneOfCase_Proof = 2,
};

GPB_FINAL @interface WaitForStateTransitionResultResponse : GPBMessage

@property(nonatomic, readonly) WaitForStateTransitionResultResponse_Responses_OneOfCase responsesOneOfCase;

@property(nonatomic, readwrite, strong, null_resettable) StateTransitionBroadcastError *error;

@property(nonatomic, readwrite, strong, null_resettable) Proof *proof;

@property(nonatomic, readwrite, strong, null_resettable) ResponseMetadata *metadata;
/** Test to see if @c metadata has been set. */
@property(nonatomic, readwrite) BOOL hasMetadata;

@end

/**
 * Clears whatever value was set for the oneof 'responses'.
 **/
void WaitForStateTransitionResultResponse_ClearResponsesOneOfCase(WaitForStateTransitionResultResponse *message);

#pragma mark - ConsensusParamsBlock

typedef GPB_ENUM(ConsensusParamsBlock_FieldNumber) {
  ConsensusParamsBlock_FieldNumber_MaxBytes = 1,
  ConsensusParamsBlock_FieldNumber_MaxGas = 2,
  ConsensusParamsBlock_FieldNumber_TimeIotaMs = 3,
};

GPB_FINAL @interface ConsensusParamsBlock : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSString *maxBytes;

@property(nonatomic, readwrite, copy, null_resettable) NSString *maxGas;

@property(nonatomic, readwrite, copy, null_resettable) NSString *timeIotaMs;

@end

#pragma mark - ConsensusParamsEvidence

typedef GPB_ENUM(ConsensusParamsEvidence_FieldNumber) {
  ConsensusParamsEvidence_FieldNumber_MaxAgeNumBlocks = 1,
  ConsensusParamsEvidence_FieldNumber_MaxAgeDuration = 2,
  ConsensusParamsEvidence_FieldNumber_MaxBytes = 3,
};

GPB_FINAL @interface ConsensusParamsEvidence : GPBMessage

@property(nonatomic, readwrite, copy, null_resettable) NSString *maxAgeNumBlocks;

@property(nonatomic, readwrite, copy, null_resettable) NSString *maxAgeDuration;

@property(nonatomic, readwrite, copy, null_resettable) NSString *maxBytes;

@end

#pragma mark - GetConsensusParamsRequest

typedef GPB_ENUM(GetConsensusParamsRequest_FieldNumber) {
  GetConsensusParamsRequest_FieldNumber_Height = 1,
  GetConsensusParamsRequest_FieldNumber_Prove = 2,
};

GPB_FINAL @interface GetConsensusParamsRequest : GPBMessage

@property(nonatomic, readwrite) int64_t height;

@property(nonatomic, readwrite) BOOL prove;

@end

#pragma mark - GetConsensusParamsResponse

typedef GPB_ENUM(GetConsensusParamsResponse_FieldNumber) {
  GetConsensusParamsResponse_FieldNumber_Block = 1,
  GetConsensusParamsResponse_FieldNumber_Evidence = 2,
};

GPB_FINAL @interface GetConsensusParamsResponse : GPBMessage

@property(nonatomic, readwrite, strong, null_resettable) ConsensusParamsBlock *block;
/** Test to see if @c block has been set. */
@property(nonatomic, readwrite) BOOL hasBlock;

@property(nonatomic, readwrite, strong, null_resettable) ConsensusParamsEvidence *evidence;
/** Test to see if @c evidence has been set. */
@property(nonatomic, readwrite) BOOL hasEvidence;

@end

NS_ASSUME_NONNULL_END

CF_EXTERN_C_END

#pragma clang diagnostic pop

// @@protoc_insertion_point(global_scope)
