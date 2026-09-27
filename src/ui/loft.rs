//! Loft launch hook — FrankenTUI Web UI (optional tray under Loft later).

use std::io::{Read, Write};
use std::net::TcpListener;

use ftui_web::step_program::StepProgram;
use ftui_web::WebBackend;

use crate::error::{Error, Result};
use crate::ui::model::HelloTick;
use crate::ui::StartupReport;

const BACKEND_ID: &str = "frankentui-web";
const PAGE: &str = "\
<!DOCTYPE html>
<html lang=\"en\">
<head><meta charset=\"utf-8\"><title>McLoad Loft</title></head>
<body>
  <h1>McLoad Loft</h1>
  <p>FrankenTUI Web is running.</p>
</body>
</html>
";

/// Non-blocking startup probe for tests / `MCLOAD_STARTUP_PROBE=1`.
///
/// Binds a local listener, initializes a FrankenTUI `WebBackend` + `StepProgram`
/// (one `init()` frame), then returns. The listener is dropped so the probe
/// does not block forever.
pub fn probe_startup() -> Result<StartupReport> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let url = format!("http://{addr}");

    let backend = WebBackend::new(80, 24);
    let mut program = StepProgram::with_backend(HelloTick::loft(), backend);
    program
        .init()
        .map_err(|e| Error::Ui(format!("FrankenTUI Web init failed: {e}")))?;

    if !program.is_initialized() {
        return Err(Error::Ui(
            "FrankenTUI Web StepProgram failed to initialize".to_string(),
        ));
    }

    // Probe must not hang — drop the listener after recording the address.
    drop(listener);

    Ok(StartupReport {
        backend: BACKEND_ID.to_string(),
        ready: true,
        detail: Some(format!("listen {url}")),
    })
}

/// Run Loft UI. With `dry_run`, return Ok without hosting Web UI (test seam).
pub fn run(dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }

    let backend = WebBackend::new(80, 24);
    let mut program = StepProgram::with_backend(HelloTick::loft(), backend);
    program
        .init()
        .map_err(|e| Error::Ui(format!("FrankenTUI Web init failed: {e}")))?;

    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    eprintln!("McLoad Loft — FrankenTUI Web listening on http://{addr}");
    eprintln!("Press Ctrl+C to stop.");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("loft accept error: {e}");
                continue;
            }
        };

        // Drain a bit of the request so clients don't hang, then respond.
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/html; charset=utf-8\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {PAGE}",
            PAGE.len()
        );
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();

        // Drive one host frame after each request (keeps StepProgram warm).
        let _ = program.step();
    }

    Ok(())
}
