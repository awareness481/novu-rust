use crate::shared::{IPartnerConfigurationResponseDto, MemberInviteDto, MemberUserDto};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrganizationDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "jobTitle", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<JobTitle>,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "productUseCases", skip_serializing_if = "Option::is_none")]
    pub product_use_cases: Option<serde_json::Value>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobTitle {
    #[serde(rename = "engineer")]
    Engineer,
    #[serde(rename = "engineering_manager")]
    EngineeringManager,
    #[serde(rename = "architect")]
    Architect,
    #[serde(rename = "product_manager")]
    ProductManager,
    #[serde(rename = "designer")]
    Designer,
    #[serde(rename = "cxo_founder")]
    CxoFounder,
    #[serde(rename = "marketing_manager")]
    MarketingManager,
    #[serde(rename = "other")]
    Other,
}

impl Default for JobTitle {
    fn default() -> JobTitle {
        Self::Engineer
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "_userId")]
    pub _user_id: String,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<MemberUserDto>>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Roles>>,
    #[serde(rename = "invite", skip_serializing_if = "Option::is_none")]
    pub invite: Option<Box<MemberInviteDto>>,
    #[serde(rename = "memberStatus", skip_serializing_if = "Option::is_none")]
    pub member_status: Option<MemberStatus>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Roles {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "member")]
    Member,
}

impl Default for Roles {
    fn default() -> Roles {
        Self::Admin
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MemberStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "invited")]
    Invited,
}

impl Default for MemberStatus {
    fn default() -> MemberStatus {
        Self::New
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationBrandingResponseDto {
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(rename = "logo")]
    pub logo: String,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "fontColor")]
    pub font_color: String,
    #[serde(rename = "contentBackground")]
    pub content_background: String,
    #[serde(rename = "fontFamily", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "ltr")]
    Ltr,
    #[serde(rename = "trl")]
    Trl,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Ltr
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationResponseDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "branding")]
    pub branding: Box<OrganizationBrandingResponseDto>,
    #[serde(
        rename = "partnerConfigurations",
        skip_serializing_if = "Option::is_none"
    )]
    pub partner_configurations: Option<Vec<IPartnerConfigurationResponseDto>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameOrganizationDto {
    #[serde(rename = "name")]
    pub name: String,
}

impl RenameOrganizationDto {
    pub fn new(name: String) -> RenameOrganizationDto {
        RenameOrganizationDto { name }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateBrandingDetailsDto {
    #[serde(rename = "logo")]
    pub logo: String,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "fontColor")]
    pub font_color: String,
    #[serde(rename = "contentBackground")]
    pub content_background: String,
    #[serde(rename = "fontFamily", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
}
