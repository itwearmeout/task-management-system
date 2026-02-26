use log::warn;
use sqlx::PgPool;
use tokio::time::{Duration, MissedTickBehavior, interval};

/// Starts a background worker that emits warnings for overdue tasks every 60 seconds.
pub fn spawn_overdue_task_monitor(pool: PgPool) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(60));
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            ticker.tick().await;

            match sqlx::query!(
                r#"
                    SELECT task_id, user_id
                    FROM user_tasks
                    WHERE due_at < NOW()
                      AND is_complete = false
                "#
            )
            .fetch_all(&pool)
            .await
            {
                Ok(tasks) => {
                    for task in tasks {
                        warn!(
                            "Overdue task detected: {} for user {}",
                            task.task_id, task.user_id
                        );
                    }
                }
                Err(err) => {
                    warn!("Failed to fetch overdue tasks: {}", err);
                }
            }
        }
    });
}
