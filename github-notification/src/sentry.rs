use sentry::ClientInitGuard;

pub fn initialize_sentry() -> ClientInitGuard {
    let sentry_dsn = std::env::var("SENTRY_DSN").expect("SENTRY_DSN is not set");
    sentry::init((
        sentry_dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ))
}
