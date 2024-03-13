pub enum KalturaUiConfObjType {
    Player = 1,
    ContributionWizard = 2,
    SimpleEditor = 3,
    AdvancedEditor = 4,
    Playlist = 5,
    AppStudio = 6,
    KRecord = 7,
    PlayerV3 = 8,
    KmcAccount = 9,
    KmcAnalytics = 10,
    KmcContent = 11,
    KmcDashboard = 12,
    KmcLogin = 13,
    PlayerSl = 14,
    ClientSideEncoder = 15,
    KmcGeneral = 16,
    KmcRolesAndPermissions = 17,
    Clipper = 18,
    Ksr = 19,
    KUpload = 20,
    Webcasting = 21,
}

pub enum KalturaUiConfCreationMode {
    WIZARD = 2,
    ADVANCED = 3,
    SYSTEM = 4,
}

pub struct KalturaUiConf {
    id: Option<i32>,
    conf_file: Option<String>,
    conf_file_params: Option<String>,
    conf_vars: Option<String>,
    config: Option<String>,
    created_at: Option<i32>,
    creation_mode: Option<KalturaUiConfCreationMode>,
    description: Option<String>,
    height: Option<String>,
    html5_url: Option<String>,
    html_params: Option<String>,
    name: Option<String>,
    obj_type: Option<KalturaUiConfObjType>,
    obj_type_as_string: Option<String>,
    partner_id: Option<i32>,
    partner_tags: Option<String>,
    swf_url: Option<String>,
    swf_url_version: Option<String>,
    tags: Option<String>,
    updated_at: Option<i32>,
    use_cdn: Option<i32>,
    version: Option<String>,
    width: Option<String>,
}

pub struct KalturaUiConfListResponse {
    obj_type: Option<String>,
    objects: Vec<KalturaUiConf>,
    total_count: i32,
}

// pub struct UiConfBaseFilter {
//     IdEqual: Option<i32>,
//     IdIn: Option<String>,
//     IdNotIn: Option<String>,
//     NameLike: Option<String>,
//     SystemNameLike: Option<String>,
//     PartnerIdEqual: Option<i32>,
//     PartnerIdIn: Option<String>,
//     PartnerIdNotIn: Option<String>,
//     ObjTypeEqual: Option<i32>,
//     ObjTypeIn: Option<String>,
//     ObjTypeNotIn: Option<String>,
//     TagsMultiLikeOr: Option<String>,
//     TagsMultiLikeAnd: Option<String>,
//     TagsNameMultiLikeOr: Option<String>,
//     TagsNameMultiLikeAnd: Option<String>,
//     OrderBy: Option<String>,
//     AdvancedSearch: Option<String>,
//     Pager: Option<String>,
// }
