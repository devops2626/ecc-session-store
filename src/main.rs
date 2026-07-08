use ecc_session_store::session::store::{SessionStore, SessionStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the store (will create/use ./sessions.db)
    let store = SessionStore::new("./sessions.db").await?;

    // Create a new session
    let session = store
        .create_session(
            "Write blog post about SQLite".into(),
            "writer-agent".into(),
            Some("/workspace".into()),
            None,
        )
        .await?;

    println!("Created session: {}", session.id);

    // Append some output
    store
        .append_output(session.id, "Generating outline...".into(), "stdout")
        .await?;

    // Mark as completed
    store.update_status(session.id, SessionStatus::Completed).await?;

    println!("Session completed!");
    Ok(())
}
