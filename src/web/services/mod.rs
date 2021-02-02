use actix_web::web::ServiceConfig;

pub mod blog;
pub mod confirm;
pub mod developers;
pub mod login;
pub mod logout;
pub mod p404;
pub mod profile;
pub mod register;

/// Register services to the actix-web server config.
pub fn register(config: &mut ServiceConfig) {
    config
        .service(login::login_get)
        .service(login::login_post)
        .service(logout::logout_service)
        .service(register::registration_service)
        .service(confirm::confirmations_page)
        .service(confirm::confirm)
        .service(register::signup_page)
        .service(profile::profile)
        .service(profile::settings_page)
        .service(profile::settings_update)
        .service(profile::add_email_page)
        .service(profile::add_email)
        .service(developers::developers_page);
}
