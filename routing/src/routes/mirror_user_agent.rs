use axum::{
    headers::{Age, UserAgent},
    TypedHeader,
};

pub async fn mirror_user_agent(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    TypedHeader(age): TypedHeader<Age>,
) -> String {
    format!(
        "User Agent is {} and Age is {}",
        user_agent.to_string(),
        age.as_secs().to_string()
    )
}
