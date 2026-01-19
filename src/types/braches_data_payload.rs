use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BhfCustSaveReq {
    pub tin: String,           // Customer TIN, max 11 chars
    pub bhfId: String,         // Branch ID, max 2 chars
    pub custNo: String,        // Customer Number, max 9 chars
    pub custTin: String,       // Customer PIN, max 11 chars
    pub custNm: String,        // Customer Name, max 60 chars
    pub adrs: Option<String>,  // Address, nullable, max 300 chars
    pub telNo: Option<String>, // Contact, nullable, max 20 chars
    pub email: Option<String>, // Email, nullable, max 50 chars
    pub faxNo: Option<String>, // Fax number, nullable, max 20 chars
    pub useYn: String,         // Used (Y/N), 1 char
    pub remark: Option<String>,// Remark, nullable, max 1000 chars
    pub regrNm: String,        // Registrant Name, max 60 chars
    pub regrId: String,        // Registrant ID, max 20 chars
    pub modrNm: String,        // Modifier Name, max 60 chars
    pub modrId: String,        // Modifier ID, max 20 chars
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BhfUserSaveReq {
    pub tin: String,            // Customer TIN / Branch TIN, max 11 chars
    pub bhfId: String,          // Branch ID, max 2 chars
    pub userId: String,         // User ID, max 20 chars
    pub userNm: String,         // User Name, max 60 chars
    pub pwd: String,            // Password, max 255 chars
    pub adrs: Option<String>,   // Address, nullable, max 200 chars
    pub cntc: Option<String>,   // Contact, nullable, max 20 chars
    pub authCd: Option<String>, // Authority Code, nullable, max 100 chars
    pub remark: Option<String>, // Remark, nullable, max 2000 chars
    pub useYn: String,          // Used/Unused (Y/N), 1 char
    pub regrNm: String,         // Registrant Name, max 60 chars
    pub regrId: String,         // Registrant ID, max 20 chars
    pub modrNm: String,         // Modifier Name, max 60 chars
    pub modrId: String,         // Modifier ID, max 20 chars
}




#[derive(Debug, Serialize, Deserialize)]
pub struct BhfInsuranceSaveReq {
    pub tin: String,           // Branch TIN, max 11 chars
    pub bhfId: String,         // Branch ID, max 2 chars
    pub isrccCd: String,       // Insurance Code, max 10 chars
    pub isrccNm: String,       // Insurance Name, max 100 chars
    pub isrcRt: i32,            // Premium Rate, max 3 digits
    pub useYn: String,         // Y/N flag, max 1 char
    pub regrNm: String,        // Registrant Name, max 60 chars
    pub regrId: String,        // Registrant ID, max 20 chars
    pub modrNm: String,        // Modifier Name, max 60 chars
    pub modrId: String,        // Modifier ID, max 20 chars
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BhfInsuranceSaveRes {
    pub resultCd: String,      // Result Code
    pub resultMsg: String,     // Result Message
    pub resultDt: String,      // Result Date/Time
}
