use rspotify_model::{PrivateUser, PublicUser};
use serde::{Deserialize, Serialize};

pub trait UserImpl {
    fn name(&self) -> String;
    fn email_safe(&self) -> String;
    fn country_safe(&self) -> String;
}

impl UserImpl for PublicUser {
    fn name(&self) -> String {
        self.display_name.clone()
            .unwrap_or(self.id.to_string())
    }

    fn email_safe(&self) -> String {
        "<none>".to_string()
    }

    fn country_safe(&self) -> String {
        "<none>".to_string()
    }
}

impl UserImpl for PrivateUser {
    fn name(&self) -> String {
        self.display_name.clone()
            .unwrap_or(self.id.to_string())
    }

    fn email_safe(&self) -> String {
        self.email.clone().unwrap_or("<none>".to_string())
    }

    fn country_safe(&self) -> String {
        self.country
            .and_then(|c| format!("{:?}", c).into())
            .unwrap_or("<none>".to_string())
    }
}