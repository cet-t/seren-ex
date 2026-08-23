#[macro_export]
macro_rules! _now {
    () => {
        chrono::Local::now().format("%Y%m%d%H:%M:%S").to_string()
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {{
        ::log::info!("📝 {} - {}", $crate::_now!(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        ::log::info!("✅ {} - {}", $crate::_now(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        ::log::warn!("⚠️ {} - {}", $crate::_now(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        ::log::error!("⛔ {} - {}", $crate::_now(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! is_bot {
    ($ctx:expr) => {
        if $ctx.author().bot {
            return Ok(());
        }
    };
}
