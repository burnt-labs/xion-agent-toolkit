//! Treasury Data Types
//!
//! Data structures for Treasury contract information and operations.
//!
//! ## Type Organization
//!
//! This module contains both CLI-specific types and chain-ready types.
//!
//! **Note on official types**: While we'd prefer to use types from the `treasury` crate,
//! the `treasury::grant` module is private, preventing us from constructing
//! `ExecuteMsg`, `GrantConfig`, or `FeeConfig` instances. Therefore, we maintain
//! our own chain-ready types (`TreasuryExecuteMsg`, `GrantConfigChain`, `FeeConfigChain`)
//! that match the contract's structure.

use serde::{Deserialize, Serialize};

/// Treasury list item (simplified view)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryListItem {
    /// Treasury contract address
    pub address: String,
    /// Admin address
    pub admin: Option<String>,
    /// Treasury balance in uxion
    pub balance: String,
    /// Display name
    #[serde(default)]
    pub name: Option<String>,
    /// Creation timestamp
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Complete treasury information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryInfo {
    /// Treasury contract address
    pub address: String,
    /// Admin address
    pub admin: Option<String>,
    /// Treasury balance in uxion
    pub balance: String,
    /// Treasury parameters
    pub params: TreasuryParams,
    /// Fee grant configuration
    #[serde(default)]
    pub fee_config: Option<FeeConfig>,
    /// Grant configurations
    #[serde(default)]
    pub grant_configs: Option<Vec<GrantConfig>>,
}

/// Treasury parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryParams {
    /// Display URL
    #[serde(default)]
    pub display_url: Option<String>,
    /// Redirect URL
    pub redirect_url: String,
    /// Icon URL
    pub icon_url: String,
    /// Additional metadata
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Fee grant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfig {
    /// Fee grant type (e.g., "basic", "limited")
    #[serde(rename = "type")]
    pub config_type: String,
    /// Maximum spend limit
    pub spend_limit: Option<String>,
    /// Expiration time
    #[serde(default)]
    pub expires_at: Option<String>,
    /// Additional configuration
    #[serde(flatten)]
    pub additional: Option<serde_json::Value>,
}

/// Grant configuration (for Authz)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfig {
    /// Type URL of the message type
    pub type_url: String,
    /// Grant configuration
    pub grant_config: serde_json::Value,
}

/// Query options for treasury details
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryOptions {
    /// Include grant configurations
    pub grants: bool,
    /// Include fee configuration
    pub fee: bool,
    /// Include admin information
    pub admin: bool,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            grants: true,
            fee: true,
            admin: true,
        }
    }
}

/// Treasury creation request
///
/// Contains all required parameters to instantiate a new treasury contract.
/// Treasury contracts are created using CosmWasm instantiate2 for predictable addresses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTreasuryRequest {
    /// Admin address (user's MetaAccount address)
    pub admin: String,

    /// Fee grant configuration (required)
    /// Allows the treasury to pay for user transactions
    pub fee_config: FeeConfigMessage,

    /// Grant configurations for Authz (required at least one)
    /// Defines what permissions the treasury can grant to users
    pub grant_configs: Vec<GrantConfigMessage>,

    /// Treasury parameters
    pub params: TreasuryParamsMessage,

    /// Treasury name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Is this an OAuth2 application treasury (optional)
    #[serde(default)]
    pub is_oauth2_app: bool,
}

/// Fee config for treasury instantiation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfigMessage {
    /// Fee allowance type URL and encoded value
    pub allowance: TypeUrlValue,
    /// Description of the fee grant
    pub description: String,
    /// Expiration timestamp as ISO 8601 string (RFC 3339 format)
    /// Optional field that will be omitted if None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
}

/// Grant config for treasury instantiation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigMessage {
    /// Message type URL (e.g., "/cosmos.bank.v1beta1.MsgSend")
    pub type_url: String,
    /// Authorization type URL and encoded value
    pub authorization: TypeUrlValue,
    /// Description of the grant (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the grant is optional
    #[serde(default)]
    pub optional: bool,
}

/// Type URL with base64-encoded value (JSON format for OAuth2 API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeUrlValue {
    /// Protobuf type URL
    pub type_url: String,
    /// Base64-encoded protobuf value (JSON format for OAuth2 API expects base64 string)
    pub value: String,
}

/// Treasury parameters for instantiation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryParamsMessage {
    /// Redirect URL for OAuth callbacks
    pub redirect_url: String,
    /// Icon URL for display
    pub icon_url: String,
    /// Display URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    /// Additional metadata as JSON object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Treasury creation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTreasuryResult {
    /// New treasury contract address
    pub treasury_address: String,
    /// Transaction hash
    pub tx_hash: String,
    /// Admin address
    pub admin: String,
    /// Creation timestamp
    pub created_at: String,
}

/// Legacy fee grant request (for backward compatibility)
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeGrantRequest {
    /// Fee grant type
    #[serde(rename = "type")]
    pub grant_type: String,
    /// Spend limit
    pub spend_limit: String,
}

/// Legacy grant config request (for backward compatibility)
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigRequest {
    /// Message type URL
    pub type_url: String,
    /// Grant configuration
    pub config: serde_json::Value,
}

// ============================================================================
// Transaction Types
// ============================================================================

/// Transaction message for broadcasting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMessage {
    /// Protobuf type URL (e.g., "/cosmos.bank.v1beta1.MsgSend")
    #[serde(rename = "typeUrl")]
    pub type_url: String,
    /// Message value as JSON object
    pub value: serde_json::Value,
}

/// Transaction broadcast request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastRequest {
    /// List of transaction messages
    pub messages: Vec<TransactionMessage>,
    /// Optional memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// Transaction broadcast response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastResponse {
    /// Success status
    pub success: bool,
    /// Transaction hash
    pub tx_hash: String,
    /// Sender address
    pub from: String,
    /// Gas used (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_used: Option<String>,
    /// Gas wanted (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_wanted: Option<String>,
}

/// Coin type for transactions
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    /// Amount (as string to handle large numbers)
    pub amount: String,
    /// Denomination (e.g., "uxion")
    pub denom: String,
}

/// Coin input for execute messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinInput {
    pub amount: String,
    pub denom: String,
}

/// Fund treasury result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundResult {
    /// Treasury address that was funded
    pub treasury_address: String,
    /// Amount funded
    pub amount: String,
    /// Transaction hash
    pub tx_hash: String,
}

/// Withdraw from treasury result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResult {
    /// Treasury address withdrawn from
    pub treasury_address: String,
    /// Amount withdrawn
    pub amount: String,
    /// Transaction hash
    pub tx_hash: String,
}

/// Result of contract instantiation (v1 - dynamic address)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantiateResult {
    /// Transaction hash
    pub tx_hash: String,
    /// Code ID of the instantiated contract
    pub code_id: u64,
    /// Label for the contract instance
    pub label: String,
    /// Admin address for contract migrations (optional)
    pub admin: Option<String>,
}

/// Result of contract instantiation2 (v2 - predictable address)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instantiate2Result {
    /// Transaction hash
    pub tx_hash: String,
    /// Code ID of the instantiated contract
    pub code_id: u64,
    /// Label for the contract instance
    pub label: String,
    /// Salt for predictable address (hex-encoded)
    pub salt: String,
    /// Admin address for contract migrations (optional)
    pub admin: Option<String>,
    /// Predicted contract address (can be computed locally)
    pub predicted_address: Option<String>,
}

/// Result of contract execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResult {
    /// Transaction hash
    pub tx_hash: String,
    /// Contract address executed
    pub contract: String,
}

// ============================================================================
// CHAIN-READY TYPES (AFTER ENCODING)
// ============================================================================

/// Treasury instantiation message (ready for blockchain submission)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryInstantiateMsg {
    pub admin: String,
    pub params: TreasuryParamsChain,
    pub fee_config: Option<FeeConfigChain>,
    pub grant_configs: Vec<GrantConfigChain>,
    #[serde(rename = "type_urls")]
    pub type_urls: Vec<String>,
}

/// Treasury parameters (chain format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryParamsChain {
    pub redirect_url: String,
    pub icon_url: String,
    pub metadata: String, // JSON string
}

/// Fee configuration (chain format)
/// Matches official treasury::grant::FeeConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfigChain {
    pub description: String,
    pub allowance: Option<ProtobufAny>,
    /// Expiration timestamp as ISO 8601 string (RFC 3339 format)
    /// The OAuth2 API expects this as a string, not a number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
}

/// Grant configuration (chain format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigChain {
    pub description: String,
    pub authorization: ProtobufAny,
    pub optional: bool,
}

/// Protobuf Any type for blockchain messages (JSON format for OAuth2 API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtobufAny {
    pub type_url: String,
    pub value: String, // Base64-encoded protobuf value (JSON format)
}

/// Create treasury response
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTreasuryResponse {
    pub treasury_address: String,
    pub transaction_hash: String,
}

// ============================================================================
// INPUT TYPES FOR CREATE COMMAND
// ============================================================================

/// Input for treasury creation (from CLI or config file)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryCreateRequest {
    pub params: TreasuryParamsInput,
    pub fee_config: Option<FeeConfigInput>,
    pub grant_configs: Vec<GrantConfigInput>,
}

/// Treasury parameters input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryParamsInput {
    pub redirect_url: String,
    pub icon_url: String,
    pub name: Option<String>,
    pub is_oauth2_app: Option<bool>,
}

/// Fee configuration input
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "allowance_type")]
pub enum FeeConfigInput {
    #[serde(rename = "basic")]
    Basic {
        spend_limit: String,
        description: String,
    },
    #[serde(rename = "periodic")]
    Periodic {
        basic_spend_limit: Option<String>,
        period_seconds: u64,
        period_spend_limit: String,
        description: String,
    },
    #[serde(rename = "allowed_msg")]
    AllowedMsg {
        allowed_messages: Vec<String>,
        nested_allowance: Box<FeeConfigInput>,
        description: String,
    },
}

/// Grant configuration input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigInput {
    #[serde(rename = "type_url")]
    pub type_url: String,
    pub description: String,
    pub authorization: AuthorizationInput,
    #[serde(default)]
    pub optional: bool,
}

/// Authorization input
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "auth_type")]
pub enum AuthorizationInput {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "send")]
    Send {
        spend_limit: String,
        allow_list: Option<Vec<String>>,
    },
    #[serde(rename = "stake")]
    Stake {
        max_tokens: String,
        validators: Option<Vec<String>>,
        deny_validators: Option<Vec<String>>,
        #[serde(default = "default_stake_auth_type")]
        authorization_type: i32, // 1=DELEGATE, 2=UNDELEGATE, 3=REDELEGATE
    },
    #[serde(rename = "ibc_transfer")]
    IbcTransfer {
        allocations: Vec<IbcAllocationInput>,
    },
    #[serde(rename = "contract_execution")]
    ContractExecution { grants: Vec<ContractGrantInput> },
}

fn default_stake_auth_type() -> i32 {
    1
}

/// IBC allocation input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IbcAllocationInput {
    pub source_port: String,
    pub source_channel: String,
    pub spend_limit: String,
    pub allow_list: Option<Vec<String>>,
}

/// Contract grant input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractGrantInput {
    pub address: String,
    pub max_calls: Option<u64>,
    pub max_funds: Option<String>,
    #[serde(default = "default_filter_type")]
    pub filter_type: String, // "allow_all" or "accepted_keys"
    pub keys: Option<Vec<String>>,
}

fn default_filter_type() -> String {
    "allow_all".to_string()
}

// ============================================================================
// GRANT CONFIG OPERATION TYPES
// ============================================================================

/// Grant config operation command
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation")]
#[allow(dead_code)]
pub enum GrantConfigOperation {
    #[serde(rename = "add")]
    Add {
        type_url: String,
        grant_config: GrantConfigInput,
    },
    #[serde(rename = "remove")]
    Remove { type_url: String },
}

/// Result of grant config operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigResult {
    /// Treasury address
    pub treasury_address: String,
    /// Type URL of the grant
    pub type_url: String,
    /// Operation performed
    pub operation: String,
    /// Transaction hash
    pub tx_hash: String,
}

/// List grant configs result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GrantConfigListResult {
    /// Treasury address
    pub treasury_address: String,
    /// List of grant configs
    pub grant_configs: Vec<GrantConfigInfo>,
}

/// Grant config info (from query)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigInfo {
    /// Type URL
    #[serde(rename = "type_url")]
    pub type_url: String,
    /// Description
    pub description: String,
    /// Authorization type URL
    pub authorization_type_url: String,
    /// Is optional
    #[serde(default)]
    pub optional: bool,
    /// Original authorization input for round-trip preservation
    /// This preserves spend limits, max_calls, validators, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_input: Option<AuthorizationInput>,
}

// ============================================================================
// FEE CONFIG OPERATION TYPES
// ============================================================================

/// Fee config operation command
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation")]
#[allow(dead_code)]
pub enum FeeConfigOperation {
    #[serde(rename = "set")]
    Set { fee_config: FeeConfigInput },
    #[serde(rename = "remove")]
    Remove {},
}

/// Result of fee config operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfigResult {
    /// Treasury address
    pub treasury_address: String,
    /// Operation performed
    pub operation: String,
    /// Transaction hash
    pub tx_hash: String,
}

/// Result of admin management operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminResult {
    /// Treasury address
    pub treasury_address: String,
    /// Operation performed
    pub operation: String,
    /// New admin address (for propose_admin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_admin: Option<String>,
    /// Transaction hash
    pub tx_hash: String,
}

/// Result of params update operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamsResult {
    /// Treasury address
    pub treasury_address: String,
    /// Transaction hash
    pub tx_hash: String,
}

/// Result of batch grant config operation
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGrantConfigResult {
    /// Treasury address
    pub treasury_address: String,
    /// Number of grant configs processed
    pub count: usize,
    /// Transaction hash
    pub tx_hash: String,
}

/// Input for update params command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParamsInput {
    /// Redirect URL for OAuth callbacks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// Icon URL for display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Treasury name (stored in metadata.name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this is an OAuth2 app (stored in metadata.is_oauth2_app)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_oauth2_app: Option<bool>,
    /// Additional metadata as JSON object (merged with name and is_oauth2_app)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Fee config info (from query)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfigInfo {
    /// Allowance type URL
    pub allowance_type_url: String,
    /// Description
    pub description: String,
    /// Spend limit (if any)
    #[serde(default)]
    pub spend_limit: Option<String>,
    /// Expiration (if any)
    #[serde(default)]
    pub expiration: Option<String>,
    /// Period duration for periodic allowances (e.g., "86400s")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// Period spend limit for periodic allowances
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_spend_limit: Option<String>,
    /// Whether the period can reset
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_period_reset: Option<bool>,
}

// ============================================================================
// EXPORT TYPES
// ============================================================================

/// Treasury configuration for export/import
///
/// This structure contains all the configuration data needed to backup
/// or migrate a treasury to a new instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryExportData {
    /// Treasury contract address
    pub address: String,

    /// Admin address
    pub admin: Option<String>,

    /// Fee configuration
    pub fee_config: Option<FeeConfigInfo>,

    /// Grant configurations
    pub grant_configs: Vec<GrantConfigInfo>,

    /// Treasury params
    pub params: Option<TreasuryParams>,

    /// Export timestamp (ISO 8601)
    pub exported_at: String,
}

// ============================================================================
// IMPORT TYPES
// ============================================================================

/// Result of treasury import operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    /// Success status
    pub success: bool,

    /// Treasury address
    pub treasury_address: String,

    /// Whether this was a dry-run
    pub dry_run: bool,

    /// List of actions performed
    pub actions: Vec<ImportAction>,

    /// Total number of transactions executed
    pub total_transactions: usize,

    /// List of errors encountered
    pub errors: Vec<String>,
}

/// Single action during import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportAction {
    /// Action type: "update_fee_config" or "update_grant_config"
    pub action_type: String,

    /// Index for grant config actions (0-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,

    /// Success status of this action
    pub success: bool,

    /// Transaction hash (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,

    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Config details (for dry-run preview)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

// ============================================================================
// QUERY RESULT TYPES (ON-CHAIN)
// ============================================================================

/// Authz grant info from on-chain query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthzGrantInfo {
    /// Granter address
    pub granter: String,
    /// Grantee address
    pub grantee: String,
    /// Authorization type URL
    pub authorization_type_url: String,
    /// Expiration time (optional)
    #[serde(default)]
    pub expiration: Option<String>,
}

/// Fee allowance info from on-chain query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeAllowanceInfo {
    /// Granter address
    pub granter: String,
    /// Grantee address
    pub grantee: String,
    /// Allowance type URL
    pub allowance_type_url: String,
    /// Spend limit (if any)
    #[serde(default)]
    pub spend_limit: Option<String>,
    /// Expiration time (optional)
    #[serde(default)]
    pub expiration: Option<String>,
    /// Period details (for periodic allowance)
    #[serde(default)]
    pub period: Option<String>,
    #[serde(default)]
    pub period_spend_limit: Option<String>,
    #[serde(default)]
    pub period_can_spend: Option<String>,
}

// ============================================================================
// BATCH OPERATION TYPES
// ============================================================================

/// Batch fund operation config file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFundConfig {
    /// List of funding operations
    pub operations: Vec<BatchFundOperation>,
}

/// Single fund operation in a batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFundOperation {
    /// Treasury contract address
    pub address: String,
    /// Amount to fund (e.g., "1000000uxion")
    pub amount: String,
}

/// Batch fund operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFundResult {
    /// Treasury address
    pub address: String,
    /// Operation status: "success" or "failed"
    pub status: String,
    /// Amount that was attempted to fund
    pub amount: String,
    /// Transaction hash (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Batch grant config file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGrantConfig {
    /// Grant type for all treasuries in this batch
    pub grant_type: String,
    /// Message type URL for the grant
    #[serde(rename = "message_type_url")]
    pub message_type_url: String,
    /// List of treasuries to configure
    pub treasuries: Vec<BatchGrantTreasury>,
    /// Optional spend limit for send authorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_limit: Option<String>,
    /// Optional description for the grant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Treasury entry in batch grant config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGrantTreasury {
    /// Treasury contract address
    pub address: String,
    /// Spend limit for this treasury (overrides global spend_limit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_limit: Option<String>,
}

/// Batch grant config operation result (per-treasury)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGrantConfigOperationResult {
    /// Treasury address
    pub address: String,
    /// Operation status: "success" or "failed"
    pub status: String,
    /// Grant type URL
    pub grant_type: String,
    /// Transaction hash (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Batch operation summary report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationSummary {
    /// Total number of operations
    pub total: usize,
    /// Number of successful operations
    pub successful: usize,
    /// Number of failed operations
    pub failed: usize,
    /// Individual operation results
    pub results: Vec<BatchOperationResult>,
}

/// Individual batch operation result (unified type for any batch operation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// Treasury address
    pub address: String,
    /// Operation status: "success" or "failed"
    pub status: String,
    /// Transaction hash (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Additional details (operation-specific)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Treasury export config for single or bulk export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryExportConfig {
    /// Export timestamp (ISO 8601)
    pub exported_at: String,
    /// Network name
    pub network: String,
    /// List of treasury configurations
    pub treasuries: Vec<TreasuryExportData>,
}

// ============================================================================
// CONTRACT EXECUTE MESSAGE TYPES
// ============================================================================
// Note: We maintain our own TreasuryExecuteMsg instead of using treasury::msg::ExecuteMsg
// because the treasury crate's grant module is private, preventing us from constructing
// GrantConfig and FeeConfig instances that ExecuteMsg requires.

/// Treasury contract execute message variants
/// Matches the contract's ExecuteMsg enum exactly
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryExecuteMsg {
    UpdateGrantConfig {
        #[serde(rename = "msg_type_url")]
        msg_type_url: String,
        grant_config: GrantConfigChain,
    },
    RemoveGrantConfig {
        #[serde(rename = "msg_type_url")]
        msg_type_url: String,
    },
    UpdateFeeConfig {
        fee_config: FeeConfigChain,
    },
    RevokeAllowance {
        grantee: String,
    },
    Withdraw {
        coins: Vec<CoinInput>,
    },
    ProposeAdmin {
        new_admin: String,
    },
    AcceptAdmin {},
    CancelProposedAdmin {},
    UpdateParams {
        params: TreasuryParamsChain,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treasury_list_item_deserialization() {
        let json = r#"{
            "address": "xion1abc123",
            "admin": "xion1def456",
            "balance": "10000000",
            "name": "My Treasury",
            "created_at": "2024-01-01T00:00:00Z"
        }"#;

        let item: TreasuryListItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.address, "xion1abc123");
        assert_eq!(item.admin, Some("xion1def456".to_string()));
        assert_eq!(item.balance, "10000000");
        assert_eq!(item.name, Some("My Treasury".to_string()));
    }

    #[test]
    fn test_treasury_list_item_minimal() {
        let json = r#"{
            "address": "xion1abc123",
            "balance": "5000000"
        }"#;

        let item: TreasuryListItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.address, "xion1abc123");
        assert_eq!(item.balance, "5000000");
        assert_eq!(item.admin, None);
        assert_eq!(item.name, None);
    }

    #[test]
    fn test_treasury_info_deserialization() {
        let json = r#"{
            "address": "xion1abc123",
            "admin": "xion1def456",
            "balance": "10000000",
            "params": {
                "display_url": "https://myapp.com",
                "redirect_url": "https://myapp.com/callback",
                "icon_url": "https://myapp.com/icon.png"
            },
            "fee_config": {
                "type": "basic",
                "spend_limit": "10000000uxion"
            },
            "grant_configs": [
                {
                    "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
                    "grant_config": {}
                }
            ]
        }"#;

        let info: TreasuryInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.address, "xion1abc123");
        assert_eq!(info.balance, "10000000");
        assert!(info.fee_config.is_some());
        assert!(info.grant_configs.is_some());
    }

    #[test]
    fn test_query_options_default() {
        let options = QueryOptions::default();
        assert!(options.grants);
        assert!(options.fee);
        assert!(options.admin);
    }

    #[test]
    fn test_fee_config_serialization() {
        let config = FeeConfig {
            config_type: "basic".to_string(),
            spend_limit: Some("1000000uxion".to_string()),
            expires_at: None,
            additional: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"type\":\"basic\""));
        assert!(json.contains("\"spend_limit\":\"1000000uxion\""));
    }

    #[test]
    fn test_treasury_create_request_deserialization() {
        let json = r#"{
            "params": {
                "redirect_url": "https://myapp.com/callback",
                "icon_url": "https://myapp.com/icon.png",
                "name": "My Treasury",
                "is_oauth2_app": true
            },
            "fee_config": {
                "allowance_type": "basic",
                "spend_limit": "1000000uxion",
                "description": "Basic fee allowance"
            },
            "grant_configs": [
                {
                    "type_url": "/cosmos.bank.v1beta1.MsgSend",
                    "description": "Allow sending funds",
                    "authorization": {
                        "auth_type": "send",
                        "spend_limit": "1000000uxion"
                    },
                    "optional": false
                }
            ]
        }"#;

        let request: TreasuryCreateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.params.redirect_url, "https://myapp.com/callback");
        assert_eq!(request.params.icon_url, "https://myapp.com/icon.png");
        assert_eq!(request.params.name, Some("My Treasury".to_string()));
        assert!(request.fee_config.is_some());
        assert_eq!(request.grant_configs.len(), 1);
    }

    #[test]
    fn test_fee_config_input_basic() {
        let json = r#"{
            "allowance_type": "basic",
            "spend_limit": "1000000uxion",
            "description": "Test"
        }"#;

        let config: FeeConfigInput = serde_json::from_str(json).unwrap();
        match config {
            FeeConfigInput::Basic {
                spend_limit,
                description,
            } => {
                assert_eq!(spend_limit, "1000000uxion");
                assert_eq!(description, "Test");
            }
            _ => panic!("Expected Basic variant"),
        }
    }

    #[test]
    fn test_fee_config_input_periodic() {
        let json = r#"{
            "allowance_type": "periodic",
            "basic_spend_limit": "10000000uxion",
            "period_seconds": 86400,
            "period_spend_limit": "1000000uxion",
            "description": "Daily limit"
        }"#;

        let config: FeeConfigInput = serde_json::from_str(json).unwrap();
        match config {
            FeeConfigInput::Periodic {
                basic_spend_limit,
                period_seconds,
                period_spend_limit,
                description,
            } => {
                assert_eq!(basic_spend_limit, Some("10000000uxion".to_string()));
                assert_eq!(period_seconds, 86400);
                assert_eq!(period_spend_limit, "1000000uxion");
                assert_eq!(description, "Daily limit");
            }
            _ => panic!("Expected Periodic variant"),
        }
    }

    #[test]
    fn test_authorization_input_send() {
        let json = r#"{
            "auth_type": "send",
            "spend_limit": "1000000uxion",
            "allow_list": ["xion1abc...", "xion1def..."]
        }"#;

        let auth: AuthorizationInput = serde_json::from_str(json).unwrap();
        match auth {
            AuthorizationInput::Send {
                spend_limit,
                allow_list,
            } => {
                assert_eq!(spend_limit, "1000000uxion");
                assert_eq!(
                    allow_list,
                    Some(vec!["xion1abc...".to_string(), "xion1def...".to_string()])
                );
            }
            _ => panic!("Expected Send variant"),
        }
    }

    #[test]
    fn test_authorization_input_stake() {
        let json = r#"{
            "auth_type": "stake",
            "max_tokens": "10000000uxion",
            "validators": ["xionvaloper1abc..."],
            "authorization_type": 1
        }"#;

        let auth: AuthorizationInput = serde_json::from_str(json).unwrap();
        match auth {
            AuthorizationInput::Stake {
                max_tokens,
                validators,
                deny_validators,
                authorization_type,
            } => {
                assert_eq!(max_tokens, "10000000uxion");
                assert_eq!(validators, Some(vec!["xionvaloper1abc...".to_string()]));
                assert_eq!(deny_validators, None);
                assert_eq!(authorization_type, 1);
            }
            _ => panic!("Expected Stake variant"),
        }
    }
}

#[test]
fn test_parse_example_config_file() {
    // Test parsing the example config file
    let config_content = r#"{
            "params": {
                "redirect_url": "https://myapp.com/callback",
                "icon_url": "https://myapp.com/icon.png",
                "name": "My Treasury",
                "is_oauth2_app": true
            },
            "fee_config": {
                "allowance_type": "basic",
                "spend_limit": "1000000uxion",
                "description": "Basic fee allowance for treasury operations"
            },
            "grant_configs": [
                {
                    "type_url": "/cosmos.bank.v1beta1.MsgSend",
                    "description": "Allow sending funds",
                    "authorization": {
                        "auth_type": "send",
                        "spend_limit": "1000000uxion"
                    },
                    "optional": false
                }
            ]
        }"#;

    let request: TreasuryCreateRequest = serde_json::from_str(config_content).unwrap();
    assert_eq!(request.params.redirect_url, "https://myapp.com/callback");
    assert_eq!(request.params.icon_url, "https://myapp.com/icon.png");
    assert_eq!(request.params.name, Some("My Treasury".to_string()));
    assert_eq!(request.params.is_oauth2_app, Some(true));

    // Check fee config
    assert!(request.fee_config.is_some());
    match request.fee_config.unwrap() {
        FeeConfigInput::Basic {
            spend_limit,
            description,
        } => {
            assert_eq!(spend_limit, "1000000uxion");
            assert_eq!(description, "Basic fee allowance for treasury operations");
        }
        _ => panic!("Expected Basic fee config"),
    }

    // Check grant configs
    assert_eq!(request.grant_configs.len(), 1);
    let grant = &request.grant_configs[0];
    assert_eq!(grant.type_url, "/cosmos.bank.v1beta1.MsgSend");
    assert_eq!(grant.description, "Allow sending funds");
    assert!(!grant.optional);
    match &grant.authorization {
        AuthorizationInput::Send {
            spend_limit,
            allow_list,
        } => {
            assert_eq!(spend_limit, "1000000uxion");
            assert!(allow_list.is_none());
        }
        _ => panic!("Expected Send authorization"),
    }
}

#[test]
fn test_update_params_input_serialization() {
    // Test with all fields
    let input = UpdateParamsInput {
        redirect_url: Some("https://example.com/callback".to_string()),
        icon_url: Some("https://example.com/icon.png".to_string()),
        name: Some("Updated Treasury".to_string()),
        is_oauth2_app: Some(true),
        metadata: Some(serde_json::json!({"custom": "value", "existing": true})),
    };

    let json = serde_json::to_string(&input).unwrap();
    assert!(json.contains("\"redirect_url\":\"https://example.com/callback\""));
    assert!(json.contains("\"icon_url\":\"https://example.com/icon.png\""));
    assert!(json.contains("\"name\":\"Updated Treasury\""));
    assert!(json.contains("\"is_oauth2_app\":true"));
    assert!(json.contains("\"custom\":\"value\""));

    // Test roundtrip
    let deserialized: UpdateParamsInput = serde_json::from_str(&json).unwrap();
    assert_eq!(
        deserialized.redirect_url,
        Some("https://example.com/callback".to_string())
    );
    assert_eq!(
        deserialized.icon_url,
        Some("https://example.com/icon.png".to_string())
    );
    assert_eq!(deserialized.name, Some("Updated Treasury".to_string()));
    assert_eq!(deserialized.is_oauth2_app, Some(true));
}

#[test]
fn test_update_params_input_partial() {
    // Test with only some fields
    let input = UpdateParamsInput {
        redirect_url: None,
        icon_url: Some("https://example.com/new-icon.png".to_string()),
        name: None,
        is_oauth2_app: Some(false),
        metadata: None,
    };

    let json = serde_json::to_string(&input).unwrap();
    assert!(!json.contains("\"redirect_url\""));
    assert!(json.contains("\"icon_url\":\"https://example.com/new-icon.png\""));
    assert!(!json.contains("\"name\""));
    assert!(json.contains("\"is_oauth2_app\":false"));

    // Test roundtrip
    let deserialized: UpdateParamsInput = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.redirect_url, None);
    assert_eq!(
        deserialized.icon_url,
        Some("https://example.com/new-icon.png".to_string())
    );
    assert_eq!(deserialized.name, None);
    assert_eq!(deserialized.is_oauth2_app, Some(false));
}

#[test]
fn test_update_params_input_minimal() {
    // Test with minimal fields
    let input = UpdateParamsInput {
        redirect_url: Some("https://example.com/callback".to_string()),
        icon_url: None,
        name: None,
        is_oauth2_app: None,
        metadata: None,
    };

    let json = serde_json::to_string(&input).unwrap();
    assert!(json.contains("\"redirect_url\""));
    assert!(!json.contains("\"icon_url\""));
    assert!(!json.contains("\"name\""));
    assert!(!json.contains("\"is_oauth2_app\""));
    assert!(!json.contains("\"metadata\""));
}

#[test]
fn test_serialize_treasury_execute_msg() {
    let msg = TreasuryExecuteMsg::UpdateGrantConfig {
        msg_type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
        grant_config: GrantConfigChain {
            description: "Test".to_string(),
            authorization: ProtobufAny {
                type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                value: "ChAKBzEwMDAwMDASBXV4aW9u".to_string(), // Base64 string directly
            },
            optional: false,
        },
    };

    let json = serde_json::to_string_pretty(&msg).unwrap();
    println!("Serialized message:\n{}", json);

    // Verify the structure matches what the contract expects
    let expected = r#"{
  "update_grant_config": {
    "msg_type_url": "/cosmos.bank.v1beta1.MsgSend",
    "grant_config": {
      "description": "Test",
      "authorization": {
        "type_url": "/cosmos.bank.v1beta1.SendAuthorization",
        "value": "ChAKBzEwMDAwMDASBXV4aW9u"
      },
      "optional": false
    }
  }
}"#;

    assert_eq!(json, expected);
}

#[test]
fn test_treasury_export_data_serialization() {
    let export_data = TreasuryExportData {
        address: "xion1abc123".to_string(),
        admin: Some("xion1admin".to_string()),
        fee_config: Some(FeeConfigInfo {
            allowance_type_url: "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
            description: "Basic fee allowance".to_string(),
            spend_limit: Some("1000000uxion".to_string()),
            expiration: None,
            period: None,
            period_spend_limit: None,
            can_period_reset: None,
        }),
        grant_configs: vec![GrantConfigInfo {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            description: "Allow sending funds".to_string(),
            authorization_type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
            optional: false,
            authorization_input: None,
        }],
        params: Some(TreasuryParams {
            display_url: None,
            redirect_url: "https://myapp.com/callback".to_string(),
            icon_url: "https://myapp.com/icon.png".to_string(),
            metadata: None,
        }),
        exported_at: "2024-01-01T00:00:00Z".to_string(),
    };

    let json = serde_json::to_string_pretty(&export_data).unwrap();
    assert!(json.contains("\"address\": \"xion1abc123\""));
    assert!(json.contains("\"admin\": \"xion1admin\""));
    assert!(json.contains("\"exported_at\": \"2024-01-01T00:00:00Z\""));
    assert!(json.contains("\"description\": \"Basic fee allowance\""));
}

#[test]
fn test_treasury_export_data_deserialization() {
    let json = r#"{
        "address": "xion1abc123",
        "admin": "xion1admin",
        "fee_config": {
            "allowance_type_url": "/cosmos.feegrant.v1beta1.BasicAllowance",
            "description": "Basic fee allowance",
            "spend_limit": "1000000uxion"
        },
        "grant_configs": [
            {
                "type_url": "/cosmos.bank.v1beta1.MsgSend",
                "description": "Allow sending funds",
                "authorization_type_url": "/cosmos.bank.v1beta1.SendAuthorization",
                "optional": false
            }
        ],
        "params": {
            "redirect_url": "https://myapp.com/callback",
            "icon_url": "https://myapp.com/icon.png"
        },
        "exported_at": "2024-01-01T00:00:00Z"
    }"#;

    let export_data: TreasuryExportData = serde_json::from_str(json).unwrap();
    assert_eq!(export_data.address, "xion1abc123");
    assert_eq!(export_data.admin, Some("xion1admin".to_string()));
    assert!(export_data.fee_config.is_some());
    assert_eq!(export_data.grant_configs.len(), 1);
    assert!(export_data.params.is_some());
    assert_eq!(export_data.exported_at, "2024-01-01T00:00:00Z");
}

#[test]
fn test_treasury_export_data_minimal() {
    let json = r#"{
        "address": "xion1abc123",
        "admin": null,
        "fee_config": null,
        "grant_configs": [],
        "params": null,
        "exported_at": "2024-01-01T00:00:00Z"
    }"#;

    let export_data: TreasuryExportData = serde_json::from_str(json).unwrap();
    assert_eq!(export_data.address, "xion1abc123");
    assert!(export_data.admin.is_none());
    assert!(export_data.fee_config.is_none());
    assert!(export_data.grant_configs.is_empty());
    assert!(export_data.params.is_none());
}

#[test]
fn test_treasury_export_data_roundtrip() {
    let original = TreasuryExportData {
        address: "xion1treasury".to_string(),
        admin: Some("xion1admin".to_string()),
        fee_config: Some(FeeConfigInfo {
            allowance_type_url: "/cosmos.feegrant.v1beta1.PeriodicAllowance".to_string(),
            description: "Periodic fee allowance".to_string(),
            spend_limit: Some("10000000uxion".to_string()),
            expiration: Some("2025-01-01T00:00:00Z".to_string()),
            period: Some("86400s".to_string()),
            period_spend_limit: Some("1000000uxion".to_string()),
            can_period_reset: Some(true),
        }),
        grant_configs: vec![
            GrantConfigInfo {
                type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                description: "Send funds".to_string(),
                authorization_type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                optional: false,
                authorization_input: Some(AuthorizationInput::Send {
                    spend_limit: "1000000uxion".to_string(),
                    allow_list: None,
                }),
            },
            GrantConfigInfo {
                type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
                description: "Execute contracts".to_string(),
                authorization_type_url: "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                optional: true,
                authorization_input: None,
            },
        ],
        params: Some(TreasuryParams {
            display_url: Some("https://myapp.com".to_string()),
            redirect_url: "https://myapp.com/callback".to_string(),
            icon_url: "https://myapp.com/icon.png".to_string(),
            metadata: Some(serde_json::json!({"name": "My Treasury"})),
        }),
        exported_at: "2024-06-15T12:30:00Z".to_string(),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&original).unwrap();

    // Deserialize back
    let deserialized: TreasuryExportData = serde_json::from_str(&json).unwrap();

    // Verify all fields match
    assert_eq!(deserialized.address, original.address);
    assert_eq!(deserialized.admin, original.admin);
    assert_eq!(deserialized.exported_at, original.exported_at);
    assert_eq!(
        deserialized.grant_configs.len(),
        original.grant_configs.len()
    );
    assert!(deserialized.fee_config.is_some());
    assert!(deserialized.params.is_some());
}

// ============================================================================
// Import Type Tests
// ============================================================================

#[test]
fn test_import_result_serialization() {
    let result = ImportResult {
        success: true,
        treasury_address: "xion1treasury".to_string(),
        dry_run: false,
        actions: vec![
            ImportAction {
                action_type: "update_fee_config".to_string(),
                index: None,
                success: true,
                tx_hash: Some("ABC123".to_string()),
                error: None,
                config: None,
            },
            ImportAction {
                action_type: "update_grant_config".to_string(),
                index: Some(0),
                success: true,
                tx_hash: Some("DEF456".to_string()),
                error: None,
                config: None,
            },
        ],
        total_transactions: 2,
        errors: vec![],
    };

    let json = serde_json::to_string_pretty(&result).unwrap();
    assert!(json.contains("\"success\": true"));
    assert!(json.contains("\"dry_run\": false"));
    assert!(json.contains("\"total_transactions\": 2"));
    assert!(json.contains("\"tx_hash\": \"ABC123\""));
}

#[test]
fn test_import_result_deserialization() {
    let json = r#"{
        "success": true,
        "treasury_address": "xion1treasury",
        "dry_run": false,
        "actions": [
            {
                "action_type": "update_fee_config",
                "success": true,
                "tx_hash": "ABC123"
            }
        ],
        "total_transactions": 1,
        "errors": []
    }"#;

    let result: ImportResult = serde_json::from_str(json).unwrap();
    assert!(result.success);
    assert!(!result.dry_run);
    assert_eq!(result.treasury_address, "xion1treasury");
    assert_eq!(result.total_transactions, 1);
    assert_eq!(result.actions.len(), 1);
}

#[test]
fn test_import_action_dry_run() {
    let action = ImportAction {
        action_type: "update_fee_config".to_string(),
        index: None,
        success: true,
        tx_hash: None,
        error: None,
        config: Some(serde_json::json!({
            "allowance_type_url": "/cosmos.feegrant.v1beta1.BasicAllowance",
            "description": "Basic fee allowance",
            "spend_limit": "1000000uxion"
        })),
    };

    let json = serde_json::to_string(&action).unwrap();
    assert!(json.contains("\"action_type\":\"update_fee_config\""));
    assert!(json.contains("\"config\":{"));
    assert!(!json.contains("tx_hash")); // Should not include null fields
}

#[test]
fn test_import_action_with_error() {
    let action = ImportAction {
        action_type: "update_grant_config".to_string(),
        index: Some(1),
        success: false,
        tx_hash: None,
        error: Some("Failed to update grant config: insufficient funds".to_string()),
        config: None,
    };

    let json = serde_json::to_string(&action).unwrap();
    assert!(json.contains("\"success\":false"));
    assert!(json.contains("\"error\":\"Failed to update grant config: insufficient funds\""));
}

#[test]
fn test_import_result_with_errors() {
    let result = ImportResult {
        success: false,
        treasury_address: "xion1treasury".to_string(),
        dry_run: false,
        actions: vec![
            ImportAction {
                action_type: "update_fee_config".to_string(),
                index: None,
                success: true,
                tx_hash: Some("ABC123".to_string()),
                error: None,
                config: None,
            },
            ImportAction {
                action_type: "update_grant_config".to_string(),
                index: Some(0),
                success: false,
                tx_hash: None,
                error: Some("Grant config update failed".to_string()),
                config: None,
            },
        ],
        total_transactions: 1,
        errors: vec!["Grant config update failed".to_string()],
    };

    assert!(!result.success);
    assert_eq!(result.total_transactions, 1);
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn test_import_result_dry_run_mode() {
    let result = ImportResult {
        success: true,
        treasury_address: "xion1treasury".to_string(),
        dry_run: true,
        actions: vec![ImportAction {
            action_type: "update_fee_config".to_string(),
            index: None,
            success: true,
            tx_hash: None,
            error: None,
            config: Some(serde_json::json!({"description": "Basic fee allowance"})),
        }],
        total_transactions: 0,
        errors: vec![],
    };

    assert!(result.dry_run);
    assert_eq!(result.total_transactions, 0); // No actual transactions in dry-run
}

#[test]
fn test_import_action_skip_none_fields() {
    // Test that None fields are skipped during serialization
    let action = ImportAction {
        action_type: "update_fee_config".to_string(),
        index: None,
        success: true,
        tx_hash: None,
        error: None,
        config: None,
    };

    let json = serde_json::to_string(&action).unwrap();

    // Verify the JSON structure
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Verify none of the optional fields are present
    assert!(!parsed.as_object().unwrap().contains_key("index"));
    assert!(!parsed.as_object().unwrap().contains_key("tx_hash"));
    assert!(!parsed.as_object().unwrap().contains_key("error"));
    assert!(!parsed.as_object().unwrap().contains_key("config"));
}
