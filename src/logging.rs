#[cfg(feature = "logging")]
pub fn init() {
    use tracing_subscriber::{fmt, EnvFilter};
    
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_target(false)
        .with_thread_ids(true)
        .init();
    
    tracing::info!("3DNTerminal v0.2.1 initialized - Logging Active");
}

#[cfg(not(feature = "logging"))]
pub fn init() {
    // No-op
}
