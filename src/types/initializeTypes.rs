use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeData {
     pub companyId: String,
    pub environmentName: String,
    pub environmentUrl: String,
    pub pin: String,
    pub branchId: String,
    pub deviceSerial: String
}