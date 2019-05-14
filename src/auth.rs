use pam::Authenticator;
use users::User;


#[derive(Debug, Clone)]
pub enum LoginError {
    AuthFailed,
    NoUser,
}

pub fn login(username: &str, password: &str) -> Result<User, LoginError> {
    use LoginError::*;

    let user = match users::get_user_by_name(&username) {
        Some(u) => u,
        None => return Err(NoUser),
    };

    // Now, setup the authenticator, we require the basic "yolm" service
    let mut authenticator =
        Authenticator::with_password("yolm").expect("Failed to init PAM client!");

    authenticator
        .get_handler()
        .set_credentials(username.clone(), password.clone());

    if let Err(_) = authenticator.authenticate() {
        return Err(AuthFailed);
    }

    authenticator.close_on_drop = false;
    authenticator
        .open_session()
        .expect("Failed to open a session!");

    Ok(user)
}
