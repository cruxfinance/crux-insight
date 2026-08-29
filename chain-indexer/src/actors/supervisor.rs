//! Process-level supervision for long-lived actors.
//!
//! On 2026-08-19 the `newBlock` ZMQ subscriber died silently and the process
//! kept running "healthy" for 10 days without indexing a single block, because
//! `main` only awaited one of the actor tasks. Every actor is now spawned via
//! [`spawn_critical`]: if any of them ends — normally, by panic, or by
//! cancellation — the process exits with status 1 and the container runtime
//! (`restart: unless-stopped`) restarts it from a clean state.

use std::future::Future;
use std::time::Duration;

use sea_orm::DbErr;
use tokio::task::JoinHandle;
use tracing::{error, warn};

/// Spawns `fut` as a tokio task and supervises it. When the task finishes for
/// any reason the whole process exits with status 1 (after flushing Sentry).
/// Returns the handle of the supervising task.
pub fn spawn_critical<F>(name: &'static str, fut: F) -> JoinHandle<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let inner = tokio::spawn(fut);
    tokio::spawn(async move {
        match inner.await {
            Ok(()) => error!("critical actor '{}' exited unexpectedly", name),
            Err(e) if e.is_panic() => error!("critical actor '{}' panicked: {:?}", name, e),
            Err(e) => error!("critical actor '{}' was cancelled: {:?}", name, e),
        }
        flush_sentry();
        std::process::exit(1);
    })
}

fn flush_sentry() {
    if let Some(client) = sentry::Hub::current().client() {
        client.flush(Some(Duration::from_secs(3)));
    }
}

/// Retries a fallible database operation with exponential backoff
/// (`base_delay * 2^n` between attempts). Gives up after `attempts` failures
/// and returns the last error. Used for the rollback delete, which can hit
/// `deadlock detected` when ci-modules concurrently insert rows referencing
/// `boxes` while the cascade delete runs.
pub async fn retry_db<T, F, Fut>(
    name: &str,
    attempts: u32,
    base_delay: Duration,
    mut op: F,
) -> Result<T, DbErr>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, DbErr>>,
{
    let mut attempt: u32 = 0;
    loop {
        match op().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                attempt += 1;
                if attempt >= attempts {
                    return Err(e);
                }
                let delay = base_delay * 2u32.saturating_pow(attempt - 1);
                warn!(
                    "{} failed (attempt {}/{}): {}; retrying in {:?}",
                    name, attempt, attempts, e, delay
                );
                tokio::time::sleep(delay).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn retry_db_succeeds_after_transient_failures() {
        let calls = Arc::new(AtomicU32::new(0));
        let c = calls.clone();
        let res: Result<u32, DbErr> = retry_db("test", 5, Duration::from_millis(1), move || {
            let c = c.clone();
            async move {
                let n = c.fetch_add(1, Ordering::SeqCst) + 1;
                if n < 3 {
                    Err(DbErr::Custom("deadlock detected".to_string()))
                } else {
                    Ok(n)
                }
            }
        })
        .await;
        assert_eq!(res.unwrap(), 3);
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn retry_db_gives_up_after_attempts() {
        let calls = Arc::new(AtomicU32::new(0));
        let c = calls.clone();
        let res: Result<u32, DbErr> = retry_db("test", 4, Duration::from_millis(1), move || {
            let c = c.clone();
            async move {
                c.fetch_add(1, Ordering::SeqCst);
                Err(DbErr::Custom("still deadlocked".to_string()))
            }
        })
        .await;
        assert!(res.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 4);
    }
}
