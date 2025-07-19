use std::sync::LazyLock;

static DEFAULT_REDIS_URL: &str = "redis://localhost:6379";
pub static REDIS_URL: LazyLock<String> =
    LazyLock::new(|| std::env::var("REDIS_URL").unwrap_or(DEFAULT_REDIS_URL.into()));

static DEFAULT_PORT: &str = "8080";
pub static PORT: LazyLock<String> =
    LazyLock::new(|| std::env::var("PORT").unwrap_or(DEFAULT_PORT.into()));

static DEFAULT_HOST: &str = "0.0.0.0";
pub static HOST: LazyLock<String> =
    LazyLock::new(|| std::env::var("HOST").unwrap_or(DEFAULT_HOST.into()));

static DEFAULT_WORKER_INTERVAL_SECONDS: u64 = 10;
pub static WORKER_INTERVAL_SECONDS: LazyLock<u64> = LazyLock::new(|| {
    std::env::var("WORKER_INTERVAL_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_WORKER_INTERVAL_SECONDS)
});

pub fn print_envs() {
    println!("HOST: {:?}", &*HOST);
    println!("PORT: {:?}", &*PORT);
    println!("REDIS_URL: {}", &*REDIS_URL);
    println!("WORKER_INTERVAL_SECONDS: {}", &*WORKER_INTERVAL_SECONDS);
}
