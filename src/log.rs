#[cfg(debug_assertions)]
#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        println!("[DEBUG] {}", $msg);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! info {
    ($msg:expr) => {};
}
