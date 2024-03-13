const DEFAULT_EXPIRY: i32 = 86400;


#[derive(Default, Debug)]
pub enum SessionType {
    #[default] USER = 0,
    ADMIN = 2,
}

#[derive(Default, Debug)]
pub struct KalturaSession {
    pub secret: String,
    pub user_id: String,
    pub partner_id: i32,
    pub expiry: i32,
    pub privileges: String,
    pub session_type: SessionType,
    pub ks: String,
}

impl KalturaSession {
    pub fn new(
        secret: String,
        user_id: String,
        partner_id: i32,
        mut expiry: i32,
        privileges: String,
        session_type: SessionType,
    ) -> KalturaSession {
        if expiry == 0 {
            expiry = DEFAULT_EXPIRY;
        }
        KalturaSession {
            secret,
            user_id,
            partner_id,
            expiry,
            privileges,
            session_type,
            ks: "".to_string(),
        }
    }
}