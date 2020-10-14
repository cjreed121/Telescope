use actix_web::web::ServiceConfig;

pub mod blog;
pub mod forgot;
pub mod login;
pub mod logout;
pub mod p404;
pub mod profile;
pub mod register;

pub fn register(config: &mut ServiceConfig) {
    config
        .service(login::login_get)
        .service(login::login_post)
        .service(logout::logout_service)
        .service(profile::profile);
}
