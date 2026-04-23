#[cfg(feature = "logging")]
pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    tracing::info!("Logging initialized");
}

#[cfg(not(feature = "logging"))]
pub fn init() {
    // No-op
}
