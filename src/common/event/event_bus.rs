use super::mitt::Mitt;
use once_cell::sync::Lazy;

/// 事件快车
///
/// ```
/// cargo add once_cell@1.18.0
/// ```
pub static EVENTBUS: Lazy<Mitt> = Lazy::new(|| {
    let mitt: Mitt = Mitt::new();
    mitt
});
