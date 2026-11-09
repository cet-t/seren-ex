#[macro_export]
macro_rules! token {
    ($key:expr) => {{
        $crate::__private::dotenvy::dotenv()
            .map_err(|e| $crate::__private::anyhow::anyhow!("Failed to load .env file: {}", e))?;
        ::std::env::var($key).to_owned().map_err(|e| {
            $crate::__private::anyhow::anyhow!("Failed to read TOKEN from environment: {}", e)
        })
    }};
    () => {
        $crate::token!("TOKEN")
    };
}

#[macro_export]
macro_rules! embed {
    ($title:expr, $author:expr, $colour:expr, $fields:expr) => {
        $crate::__private::serenity::all::CreateEmbed::default()
            .title($title)
            .colour($colour)
            .author($crate::__private::serenity::all::CreateEmbedAuthor::new(
                $author,
            ))
            .fields($fields.iter().cloned())
    };
}

#[macro_export]
macro_rules! reply {
    () => {
        $crate::__private::poise::CreateReply::default()
    };
    ($($arg:tt)*) => {
        $crate::__private::poise::CreateReply::default().content(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! edit {
    ($msg:expr, $ctx:expr, $reply:expr) => {{ $msg.edit($ctx, $reply) }};
}

#[macro_export]
macro_rules! call {
    ($mentionable:expr) => {{
        use $crate::__private::poise::serenity_prelude::Mentionable as _;
        $mentionable.mention()
    }};
}

#[macro_export]
macro_rules! wait {
    ($unit:ident $duration:expr) => {{
        use $crate::__private::tokio::time::{Duration, sleep};

        match stringify!($unit) {
            "s" | "sec" => sleep(Duration::from_secs_f64($duration as f64)),
            "ms" => sleep(Duration::from_millis($duration as u64)),
            "us" => sleep(Duration::from_micros($duration as u64)),
            "ns" => sleep(Duration::from_nanos($duration as u64)),
            _ => sleep(Duration::ZERO),
        }
    }};
}

#[macro_export]
macro_rules! code {
    ($($t:tt)*) => {
        format!("`{}`", format!($($t)*))
    };
}

#[macro_export]
macro_rules! icode {
    ($($t:tt)*) => {
        format!("```{}```", format!($($t)*))
    };
}
