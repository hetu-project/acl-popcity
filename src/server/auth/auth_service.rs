use oauth2::CsrfToken;

pub fn gen_csrf_token() -> String {
    CsrfToken::new_random().secret().to_string()
}
