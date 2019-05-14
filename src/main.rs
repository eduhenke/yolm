use std::os::unix::process::CommandExt;
use std::process::Command;
use users::User;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;
mod auth;

// A simple program that requests a login and a password and then spawns /bin/bash as the
// user who logged in.
//
// Note that this proto-"sudo" is very insecure and should not be used in any production setup,
// it is just an example to show how the PAM api works.
fn main() {
    let mut siv = Cursive::default();

    let username_input = LinearLayout::horizontal()
        .child(TextView::new("username: "))
        .child(
            EditView::new()
                .on_submit(handle_username)
                .with_id("username")
                .fixed_width(20),
        );

    let password_input = LinearLayout::horizontal()
        .child(TextView::new("password: "))
        .child(
            EditView::new()
                .secret()
                .on_submit(handle_password)
                .with_id("password")
                .fixed_width(20),
        );

    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(username_input)
            .child(password_input),
    ));

    siv.run();
}

fn try_login(s: &mut Cursive, username: &str, password: &str) {
    match auth::login(username, password) {
        Ok(user) => {
            spawn_sway(user);
            s.add_layer(Dialog::info("Want to login again?"));
        },
        Err(e) => {
            let msg = match e {
                auth::LoginError::AuthFailed => "Invalid credentials",
                auth::LoginError::NoUser => "Non-existing user",
            };
            s.add_layer(Dialog::info(msg));
        }
    }
}

fn handle_username(s: &mut Cursive, field: &str) {
    if field.is_empty() {
        s.add_layer(Dialog::info("Please enter the username!"));
    } else {
        let password = s.call_on_id("password", |view: &mut EditView| {
            view.get_content()
        }).unwrap().to_string();
        try_login(s, field, &password);
    }
}

fn handle_password(s: &mut Cursive, field: &str) {
    if field.is_empty() {
        s.add_layer(Dialog::info("Please enter the password!"));
    } else {
        let username = s.call_on_id("username", |view: &mut EditView| {
            view.get_content()
        }).unwrap().to_string();
        try_login(s, &username, field);
    }
}

fn spawn_sway(user: User) {
    // we now try to spawn `/usr/bin/sway` as this user
    // note that setting the uid/gid is likely to fail if this program is not already run as the
    // proper user or as root
    let sway_call = Command::new("/usr/bin/sway")
        .env("XDG_RUNTIME_DIR", format!("/run/user/{}", user.uid()))
        .uid(user.uid())
        .gid(user.primary_group_id())
        .spawn();

    let _sway_proc = match sway_call {
        Ok(p) => p,
        Err(e) => panic!("error on calling sway: {:?}", e),
    };
}