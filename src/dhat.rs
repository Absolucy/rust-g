use dhat::Profiler;
use std::sync::{LazyLock, Mutex};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

static PROFILER: LazyLock<Mutex<Option<Profiler>>> = LazyLock::new(Mutex::default);

byond_fn!(fn start_dhat(path) {
    let mut profiler = PROFILER.lock().unwrap();
    if let Some(old_profiler) = profiler.take() {
        std::mem::drop(old_profiler);
    }
    *profiler = Some(Profiler::builder().file_name(path).build());
    Some("")
});

byond_fn!(
    fn stop_dhat() {
        std::mem::drop(PROFILER.lock().unwrap().take());
        Some("")
    }
);

#[ctor::dtor]
fn ensure_dhat_stopped() {
    if let Ok(mut profiler) = PROFILER.try_lock() {
        std::mem::drop(profiler.take());
    };
}
