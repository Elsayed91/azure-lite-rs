//! Integration smoke test for Azure AD authentication.
//!
//! Verifies that the credential chain can acquire a valid token.
//! Requires Azure credentials in the environment.

#[tokio::test]
#[ignore = "requires Azure credentials"]
async fn auth_acquire_token_and_verify_non_empty() {
    println!("[1/3] Resolving Azure credential chain...");
    let cred = azure_lite::default_credential()
        .await
        .expect("Failed to resolve Azure credentials");

    println!("[2/3] Acquiring access token...");
    let token = cred
        .get_token()
        .await
        .expect("Failed to acquire access token");

    assert!(!token.token.is_empty(), "access_token must be non-empty");

    let remaining = token.seconds_remaining();
    println!(
        "[3/3] Token acquired successfully — {} chars, expires in {} seconds (~{:.0} minutes)",
        token.token.len(),
        remaining,
        remaining as f64 / 60.0,
    );

    assert!(
        remaining > 60,
        "token should have at least 60 seconds remaining, got {remaining}s"
    );

    println!("✓ Auth smoke test passed");
}

#[tokio::test]
#[ignore = "requires Azure credentials"]
async fn client_from_env_acquires_token() {
    println!("[1/2] Building AzureHttpClient from environment...");
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    println!("[2/2] Acquiring token through client...");
    let token = client.token().await.expect("Failed to acquire token");

    assert!(!token.is_empty(), "token must be non-empty");
    println!(
        "✓ Client token acquisition test passed ({} chars)",
        token.len()
    );
}
