#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMxRecordResponseDto {
    #[serde(rename = "mxRecordConfigured")]
    pub mx_record_configured: bool,
}

impl GetMxRecordResponseDto {
    pub fn new(mx_record_configured: bool) -> GetMxRecordResponseDto {
        GetMxRecordResponseDto {
            mx_record_configured,
        }
    }
}
