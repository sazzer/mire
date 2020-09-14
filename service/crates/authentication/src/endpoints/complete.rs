use crate::{service::AuthenticationService, ProviderId};
use actix_web::{
    web::{Data, Path, Query},
    HttpResponse,
};
use mire_authorization::AuthorizationService;
use std::collections::HashMap;
use string_template::Template;

const SUCCESS_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Mire</title>
</head>
<body>
    <div>Successfully authenticated as {{displayName}}</div>
    <div>Access Token: {{accessToken}}</div>
    <div>Expires: {{expires}}</div>
    <div>User ID: {{userId}}</div>

    <script>
        const message = {
            type: 'mireAuthenticated',
            accessToken: '{{accessToken}}',
            expires: new Date('{{expires}}'),
            user: '{{userId}}',
            name: '{{displayName}}',
        };
        console.log(message);
        window.opener.postMessage(message, '*');
        window.close();
    </script>
</body>
</html>
"#;
const FAILURE_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Mire</title>
</head>
<body>
    <div>Authentication failed</div>
</body>
</html>
"#;

/// HTTP Handler for completing authentication with the desired provider
///
/// # Parameters
/// - `path` - The path parameters including the provider ID
/// - `query` - The set of query parameters needed to complete authentication
/// - `authentication_service` - The Authentiaction Service
///
/// # Returns
/// The HTTP Model for the response
#[tracing::instrument(
    fields(http_method = "GET", http_path = "/authentication/:provider/complete"),
    skip(authentication_service, authorization_service)
)]
pub async fn complete(
    path: Path<ProviderId>,
    query: Query<HashMap<String, String>>,
    authentication_service: Data<AuthenticationService>,
    authorization_service: Data<AuthorizationService>,
) -> HttpResponse {
    let user_result = authentication_service
        .complete_authentication(&path.0, &query.0)
        .await;

    if let Ok(user) = user_result {
        let security_context =
            authorization_service.generate_security_context(user.identity.id.clone().into());
        let signed_security_context = authorization_service.sign(&security_context);

        let user_id_str = user.identity.id.to_string();
        let expires_str = security_context.not_valid_after.to_rfc3339();

        let template = Template::new(SUCCESS_TEMPLATE);
        let mut vals = HashMap::new();
        vals.insert("accessToken", signed_security_context.as_ref());
        vals.insert("expires", expires_str.as_ref());
        vals.insert("userId", user_id_str.as_ref());

        HttpResponse::Ok()
            .content_type("text/html;charset=utf-8")
            .body(template.render(&vals))
    } else {
        let template = Template::new(FAILURE_TEMPLATE);
        let vals = HashMap::new();

        HttpResponse::BadRequest()
            .content_type("text/html;charset=utf-8")
            .body(template.render(&vals))
    }
}
