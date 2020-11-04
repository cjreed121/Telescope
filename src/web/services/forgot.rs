
use actix_web::{
    HttpResponse,
    web::Form
};

use crate::{
    templates::{
        recovery::PasswordRecoveryPage,
        page::Page
    },
    web::RequestContext
};
use crate::models::Email;

/// Form submitted by users to recovery service to set a new password.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordRecoveryForm {
    email: String,
}

/// The password recovery page.
#[get("/forgot")]
pub async fn forgot_page(ctx: RequestContext) -> HttpResponse {
    let form = PasswordRecoveryPage::default();
    let page = Page::of("Forgot Password", &form, &ctx).await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(ctx.render(&page))
}

#[post("/forgot")]
pub async fn recovery_service(ctx: RequestContext, form: Form<PasswordRecoveryForm>) -> HttpResponse {
    let email: &str = &form.email;
    let db_conn = ctx.get_db_conn().await;
    let mut form_page = PasswordRecoveryPage::default()
        .email(email);
    let database_result = Email::get_user_from_db_by_email(db_conn, email.to_string())
        .await;
    if let Some(target_user) = database_result {
        unimplemented!()
    } else {
        form_page = form_page.error("Email Not Found");
        let page = ctx.render_in_page(&form_page, "Forgot Password").await;
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(page)
    }
}
