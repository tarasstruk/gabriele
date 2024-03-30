use tokio::time;

pub async fn wait_long() {
    wait(1000).await
}

pub async fn wait_short() {
    wait(200).await
}

pub async fn wait_tiny() {
    wait(50).await
}
pub async fn wait(millis: u64) {
    let mut interval = time::interval(time::Duration::from_millis(millis));
    interval.tick().await;
}
