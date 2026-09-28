//! Loft launch — FrankenTUI Web host + optional tray under Loft (not a CLI mode).

use std::thread;
use std::time::Duration;

use crate::error::Result;
use crate::ui::loft_host;
use crate::ui::StartupReport;

const BACKEND_ID: &str = "frankentui-web";

/// Options for Loft launch (from CLI `--no-tray` / `--verbose`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LoftOptions {
    /// Force foreground web server (also used when OS has no tray).
    pub no_tray: bool,
    /// Allow CLI stdout/stderr (default loft+tray is quiet).
    pub verbose: bool,
}

/// Whether this OS should default to tray+BG under Loft.
///
/// Windows / macOS: yes. Linux / WSL / headless: use `--no-tray` (or default foreground).
pub fn tray_supported() -> bool {
    cfg!(windows) || cfg!(target_os = "macos")
}

/// Effective tray use: supported OS and not `--no-tray`.
pub fn should_use_tray(opts: LoftOptions) -> bool {
    tray_supported() && !opts.no_tray
}

/// Non-blocking startup probe for tests / `MCLOAD_STARTUP_PROBE=1`.
pub fn probe_startup() -> Result<StartupReport> {
    let (_addr, detail) = loft_host::probe()?;
    Ok(StartupReport {
        backend: BACKEND_ID.to_string(),
        ready: true,
        detail: Some(detail),
    })
}

/// Run Loft UI. With `dry_run`, return Ok without hosting Web UI (test seam).
pub fn run(dry_run: bool, opts: LoftOptions) -> Result<()> {
    if dry_run {
        return Ok(());
    }

    if should_use_tray(opts) {
        run_with_tray(opts)
    } else {
        run_foreground(opts)
    }
}

fn run_foreground(opts: LoftOptions) -> Result<()> {
    // Foreground: always print listen URL so the user can open the UI.
    // Primary quit is `q` in the UI; Ctrl+C remains an interrupt escape hatch.
    let verbose = true;
    let _ = opts;
    loft_host::run_until_quit(verbose)?;
    Ok(())
}

fn run_with_tray(opts: LoftOptions) -> Result<()> {
    let (listener, handle) = loft_host::bind_loft("127.0.0.1:0")?;
    if opts.verbose {
        eprintln!(
            "McLoad Loft — FrankenTUI Web listening on {} (tray+BG)",
            handle.url()
        );
        eprintln!("Press q in the UI (or tray Quit) to stop the host.");
    }

    let handle_bg = handle.clone();
    let join = thread::spawn(move || loft_host::run_loop(listener, handle_bg));

    #[cfg(any(windows, target_os = "macos"))]
    {
        tray::run_tray_until_quit(handle, opts.verbose)?;
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    {
        let _ = opts;
        // Unreachable when should_use_tray is correct; keep process alive until host stops.
        while !handle.is_stopped() {
            thread::sleep(Duration::from_millis(100));
        }
    }

    let _ = join.join();
    Ok(())
}

#[cfg(any(windows, target_os = "macos"))]
mod tray {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    use tao::event_loop::{ControlFlow, EventLoopBuilder};
    use tao::platform::run_return::EventLoopExtRunReturn;
    use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
    use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};

    pub fn run_tray_until_quit(handle: loft_host::WebHostHandle, _verbose: bool) -> Result<()> {
        let icon = make_icon();
        let menu = Menu::new();
        let quit_item = MenuItem::new("Quit", true, None);
        let _ = menu.append(&PredefinedMenuItem::separator());
        let _ = menu.append(&quit_item);
        let quit_id = quit_item.id().clone();

        let _tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("McLoad Loft")
            .with_icon(icon)
            .build()
            .map_err(|e| crate::error::Error::Ui(format!("tray icon failed: {e}")))?;

        let mut event_loop = EventLoopBuilder::new().build();
        let menu_channel = MenuEvent::receiver();
        let tray_channel = TrayIconEvent::receiver();
        let done = AtomicBool::new(false);

        let _exit_code = event_loop.run_return(move |_event, _, control_flow| {
            *control_flow = ControlFlow::WaitUntil(
                std::time::Instant::now() + Duration::from_millis(200),
            );

            if handle.is_stopped() || done.load(Ordering::SeqCst) {
                *control_flow = ControlFlow::Exit;
                return;
            }

            while let Ok(event) = menu_channel.try_recv() {
                if event.id == quit_id {
                    handle.request_stop();
                    done.store(true, Ordering::SeqCst);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            while let Ok(_event) = tray_channel.try_recv() {
                // Left-click etc. — no-op for scaffolding beyond Quit menu.
            }
        });
        Ok(())
    }

    fn make_icon() -> Icon {
        // Simple 16x16 solid icon (RGBA).
        let size = 16u32;
        let mut rgba = Vec::with_capacity((size * size * 4) as usize);
        for y in 0..size {
            for x in 0..size {
                let edge = x == 0 || y == 0 || x == size - 1 || y == size - 1;
                if edge {
                    rgba.extend_from_slice(&[0x2f, 0x81, 0xf7, 0xff]);
                } else {
                    rgba.extend_from_slice(&[0x0b, 0x0d, 0x10, 0xff]);
                }
            }
        }
        Icon::from_rgba(rgba, size, size).expect("valid icon")
    }
}
