use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;
mod auth;
mod sway;

// YoLM - a simple program that requests user's name and password, logins and spawn sway
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
            let msg = match sway::spawn(user) {
                Ok(_) => "Want to login again?",
                Err(_) => "Error on spawning sway :(\nAre you sure that sway is in /usr/bin/sway?",
            };
            s.add_layer(Dialog::info(msg));
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