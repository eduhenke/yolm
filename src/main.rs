use std::io;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::Command;
use pam::Authenticator;
use users::User;
mod input;

// A simple program that requests a login and a password and then spawns /bin/bash as the
// user who logged in.
//
// Note that this proto-"sudo" is very insecure and should not be used in any production setup,
// it is just an example to show how the PAM api works.

#[derive(Debug, Clone)]
// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
enum LoginError {
    AuthFailed,
    NoUser,
}



fn login(username: String, password: String) -> Result<User, LoginError> {
    use LoginError::*;

    let user = match users::get_user_by_name(&username) {
        Some(u) => u,
        None => return Err(NoUser),
    };

    // Now, setup the authenticator, we require the basic "system-auth" service
    let mut authenticator = 
        Authenticator::with_password("system-auth").expect("Failed to init PAM client!");

    authenticator.get_handler().set_credentials(username, password);

    if let Err(_) = authenticator.authenticate() {
        return Err(AuthFailed);
    }

    authenticator
        .open_session()
        .expect("Failed to open a session!");
    
    Ok(user)
}

fn main() {
    // First, prompt the user for a login and a password
    print!("login: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    username.pop(); // remove the trailing '\n'
    print!("password: ");
    io::stdout().flush().unwrap();
    let password =
        input::read_password_from_tty().expect("could not read password");

    let user = login(username, password).expect("Login Failed");

    // we now try to spawn `/usr/bin/sway` as this user
    // note that setting the uid/gid is likely to fail if this program is not already run as the
    // proper user or as root
    let error = Command::new("/usr/bin/sway")
        .uid(user.uid())
        .gid(user.primary_group_id())
        .exec();
    match error.kind() {
        io::ErrorKind::NotFound => println!("you don't have sway installed"),
        e => println!("other error {:?}", e)
    }
}
