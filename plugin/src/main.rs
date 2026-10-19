use nih_plug::prelude::*;

use slackwater::SpectralCompressor;

fn main() {
    // macOS only lets the process's main thread create a window. Anywhere else AppKit throws an
    // Objective-C exception, which Rust cannot unwind through, so the whole process aborts
    // ("Rust cannot catch foreign exceptions"). The main thread already has an 8 MB stack there.
    #[cfg(target_os = "macos")]
    nih_export_standalone::<SpectralCompressor>();

    // Elsewhere, the deeply nested GUI widget tree can overflow the default stack size in debug
    // builds, so run the standalone on a thread with a larger stack.
    #[cfg(not(target_os = "macos"))]
    {
        let builder = std::thread::Builder::new()
            .name("main".to_string())
            .stack_size(8 * 1024 * 1024); // 8 MB
        let handler = builder
            .spawn(|| {
                nih_export_standalone::<SpectralCompressor>();
            })
            .expect("Failed to spawn main thread");
        handler.join().unwrap();
    }
}
