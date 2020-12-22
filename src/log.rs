#[derive(Debug)]
#[repr(C)]
pub enum LogLevel {
    NotSet = -1,
    None = 0,
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Perf,
    Config,
    Debug,
}

#[macro_export(local_inner_macros)]
macro_rules! function {
    () => {{
        "<wasm-rust>"
    }};
}

#[allow(unused_unsafe)]
#[macro_export]
macro_rules!do_log {
    ($level:expr, $code:expr, $($arg:tt)*) => ({
        let s = format!($($arg)*);
        unsafe{
            $crate::hc_log(
                $level as i32,
                file!(),
                line!(),
                $crate::function!(),
                $code,
                &s)
        };
    })
}

#[macro_export]
macro_rules!SCLogError {
    ($($arg:tt)*) => {
        $crate::do_log!($crate::LogLevel::Error, 0, $($arg)*);
    };
}

#[macro_export]
macro_rules!SCLogNotice {
    ($($arg:tt)*) => {
        $crate::do_log!($crate::LogLevel::Notice, 0, $($arg)*);
    }
}

#[macro_export]
macro_rules!SCLogInfo {
    ($($arg:tt)*) => {
        $crate::do_log!($crate::LogLevel::Info, 0, $($arg)*);
    }
}

#[macro_export]
macro_rules!SCLogPerf {
    ($($arg:tt)*) => {
        $crate::do_log!($crate::LogLevel::Perf, 0, $($arg)*);
    }
}

#[macro_export]
macro_rules!SCLogConfig {
    ($($arg:tt)*) => {
        $crate::do_log!($crate::LogLevel::Config, 0, $($arg)*);
    }
}

// Debug mode: call C SCLogDebug
#[cfg(feature = "debug")]
#[macro_export]
macro_rules!SCLogDebug {
    ($($arg:tt)*) => {
        $crate::do_log!($crate::LogLevel, 0, $($arg)*);
    }
}

// SCLogDebug variation to use when not compiled with debug support.
//
// This macro will only use the parameters passed to prevent warnings
// about unused variables, but is otherwise the equivalent to a no-op.
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules!SCLogDebug {
    ($last:expr) => { let _ = &$last; let _ = $crate::log::Level::Debug; };
    ($one:expr, $($arg:tt)*) => { let _ = &$one; $crate::SCLogDebug!($($arg)*); };
}

