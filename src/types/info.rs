use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationInfo{
tin: String,
bhfId: String,
lastReqDt: String,
} 