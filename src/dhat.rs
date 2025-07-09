use dhat::Profiler;
use std::sync::{LazyLock, Mutex};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

static PROFILER: LazyLock<Mutex<Option<Profiler>>> = LazyLock::new(Mutex::default);

pub fn start_dhat(path: String) {
    let mut profiler = PROFILER.lock().unwrap();
    if let Some(old_profiler) = profiler.take() {
        std::mem::drop(old_profiler);
    }
    *profiler = Some(Profiler::builder().file_name(path).build());
}

pub fn stop_dhat() {
    std::mem::drop(PROFILER.lock().unwrap().take());
}
