use sentry::ClientInitGuard;

pub fn initialize_sentry() -> Option<ClientInitGuard> {
    match std::env::var("SENTRY_DSN") {
      Ok(sentry_dsn) => {
        let guard = sentry::init((
            sentry_dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));
        Some(guard)
      },
      _ => {
        println!("SENTRY_DSN is not set");
        None
      }
    }
}
