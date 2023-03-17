use actix_session::Session as ActixSession;
use uuid::Uuid;

const SESSION_USER: &str = "auth-user";

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionUser {
    pub id: Uuid,
    pub email: String,
}

pub struct SessionClient {
    session: ActixSession,
}

impl From<ActixSession> for SessionClient {
    fn from(session: ActixSession) -> Self {
        Self { session }
    }
}

impl SessionClient {
    /// Set user into session
    pub fn set_user(&self, id: &Uuid, email: &str) {
        debug!("SET {SESSION_USER}: {id}, {email}");
        if let Err(err) = self.session.insert(
            SESSION_USER,
            SessionUser {
                id: id.clone(),
                email: email.to_string(),
            },
        ) {
            error!("SET ERROR: {err}");
        }
    }

    /// Retrieve user from session
    pub fn get_user(&self) -> Option<SessionUser> {
        debug!("GET {SESSION_USER}");
        self.session.get(SESSION_USER).unwrap()
    }
}
