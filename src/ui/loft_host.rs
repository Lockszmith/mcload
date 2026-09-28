//! Minimal native Web host for FrankenTUI `StepProgram`.
//!
//! `ftui-web` is host-driven (no HTTP server). This module binds a local
//! listener, serves a tiny page that renders the Model cell buffer, forwards
//! key events into `StepProgram`, and returns when the Model yields `Cmd::Quit`
//! (so Loft can exit the process with code 0).

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ftui::core::event::{Event, KeyCode, KeyEvent};
use ftui::render::buffer::Buffer;
use ftui_web::step_program::StepProgram;
use ftui_web::WebBackend;

use crate::error::{Error, Result};
use crate::ui::model::HelloTick;

const COLS: u16 = 80;
const ROWS: u16 = 24;

const PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>McLoad Loft</title>
<style>
  html, body { margin: 0; height: 100%; background: #0b0d10; color: #e6edf3; }
  #frame {
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 14px; line-height: 1.25;
    white-space: pre; margin: 1rem; padding: 0;
  }
  #hint { margin: 0 1rem 1rem; opacity: 0.7; font: 13px system-ui, sans-serif; }
</style>
</head>
<body>
  <pre id="frame">Loading…</pre>
  <p id="hint">Same FrankenTUI Model as Croft — press <kbd>q</kbd> to quit (stops host).</p>
  <script>
    const el = document.getElementById('frame');
    async function refresh() {
      try {
        const r = await fetch('/frame', { cache: 'no-store' });
        if (!r.ok) return;
        el.textContent = await r.text();
      } catch (_) {}
    }
    async function sendKey(key) {
      try {
        await fetch('/key', { method: 'POST', headers: { 'Content-Type': 'text/plain' }, body: key });
        await refresh();
      } catch (_) {}
    }
    window.addEventListener('keydown', (e) => {
      if (e.key === 'q' || e.key === 'Q') {
        e.preventDefault();
        sendKey('q');
      }
    });
    setInterval(refresh, 200);
    refresh();
  </script>
</body>
</html>
"#;

struct HostInner {
    program: StepProgram<HelloTick>,
    frame_text: String,
}

/// Shared handle used by the accept loop (and optional tray Quit).
#[derive(Clone)]
pub struct WebHostHandle {
    inner: Arc<Mutex<HostInner>>,
    stop: Arc<AtomicBool>,
    addr: SocketAddr,
}

impl WebHostHandle {
    pub fn local_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    /// Request host shutdown (e.g. tray Quit). Process should exit after `run` returns.
    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Ok(mut guard) = self.inner.lock() {
            // Push quit through the Model so parity with UI `q` holds.
            let _ = guard
                .program
                .push_event(Event::Key(KeyEvent::new(KeyCode::Char('q'))));
            let _ = guard.program.step();
            refresh_frame(&mut guard);
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.stop.load(Ordering::SeqCst)
            || self
                .inner
                .lock()
                .map(|g| !g.program.is_running())
                .unwrap_or(true)
    }
}

/// Bind + init `StepProgram` with the shared Loft Model (does not accept forever).
pub fn bind_loft(addr: &str) -> Result<(TcpListener, WebHostHandle)> {
    let backend = WebBackend::new(COLS, ROWS);
    let mut program = StepProgram::with_backend(HelloTick::loft(), backend);
    program
        .init()
        .map_err(|e| Error::Ui(format!("FrankenTUI Web init failed: {e}")))?;

    let mut inner = HostInner {
        program,
        frame_text: String::new(),
    };
    refresh_frame(&mut inner);

    let listener = TcpListener::bind(addr)?;
    listener.set_nonblocking(true)?;
    let sock_addr = listener.local_addr()?;

    let handle = WebHostHandle {
        inner: Arc::new(Mutex::new(inner)),
        stop: Arc::new(AtomicBool::new(false)),
        addr: sock_addr,
    };
    Ok((listener, handle))
}

/// Run the accept loop until Model quits (`q`) or `request_stop`.
pub fn run_loop(listener: TcpListener, handle: WebHostHandle) -> Result<()> {
    while !handle.is_stopped() {
        match listener.accept() {
            Ok((stream, _)) => {
                if let Err(e) = handle_connection(stream, &handle) {
                    tracing::debug!("loft connection error: {e}");
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(e) => {
                tracing::warn!("loft accept error: {e}");
                std::thread::sleep(Duration::from_millis(50));
            }
        }
    }
    Ok(())
}

/// Convenience: bind ephemeral port, run until quit (no browser / URL side effects).
pub fn run_until_quit() -> Result<SocketAddr> {
    let (listener, handle) = bind_loft("127.0.0.1:0")?;
    let addr = handle.local_addr();
    run_loop(listener, handle)?;
    Ok(addr)
}

fn refresh_frame(inner: &mut HostInner) {
    if let Some(buf) = inner.program.outputs().last_buffer.as_ref() {
        inner.frame_text = buffer_to_text(buf);
    } else {
        // Force a present by stepping a tick if buffer missing after init.
        let _ = inner.program.step();
        if let Some(buf) = inner.program.outputs().last_buffer.as_ref() {
            inner.frame_text = buffer_to_text(buf);
        }
    }
}

fn buffer_to_text(buf: &Buffer) -> String {
    let mut out = String::with_capacity(usize::from(buf.width()) * usize::from(buf.height()) + 24);
    for y in 0..buf.height() {
        for x in 0..buf.width() {
            let cell = buf.get_unchecked(x, y);
            if cell.is_continuation() {
                continue;
            }
            out.push(cell.content.as_char().unwrap_or(' '));
        }
        out.push('\n');
    }
    out
}

fn handle_connection(mut stream: TcpStream, handle: &WebHostHandle) -> Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;

    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf)?;
    if n == 0 {
        return Ok(());
    }
    let req = String::from_utf8_lossy(&buf[..n]);
    let mut lines = req.lines();
    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");

    match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => {
            write_response(&mut stream, "text/html; charset=utf-8", PAGE.as_bytes())?;
        }
        ("GET", "/frame") => {
            let body = handle
                .inner
                .lock()
                .map(|g| g.frame_text.clone())
                .unwrap_or_default();
            write_response(&mut stream, "text/plain; charset=utf-8", body.as_bytes())?;
        }
        ("POST", "/key") => {
            let body = extract_body(&req, &buf[..n]);
            let key = body.chars().next().unwrap_or('\0');
            if key != '\0' {
                let mut guard = handle
                    .inner
                    .lock()
                    .map_err(|_| Error::Ui("loft host lock poisoned".into()))?;
                let _ = guard
                    .program
                    .push_event(Event::Key(KeyEvent::new(KeyCode::Char(key))));
                let result = guard
                    .program
                    .step()
                    .map_err(|e| Error::Ui(format!("FrankenTUI Web step failed: {e}")))?;
                refresh_frame(&mut guard);
                if !result.running {
                    handle.stop.store(true, Ordering::SeqCst);
                }
            }
            write_response(&mut stream, "text/plain; charset=utf-8", b"ok")?;
        }
        _ => {
            write_status(&mut stream, "404 Not Found", b"not found")?;
        }
    }
    Ok(())
}

fn extract_body(req: &str, raw: &[u8]) -> String {
    if let Some(idx) = req.find("\r\n\r\n") {
        return String::from_utf8_lossy(&raw[idx + 4..]).trim().to_string();
    }
    if let Some(idx) = req.find("\n\n") {
        return String::from_utf8_lossy(&raw[idx + 2..]).trim().to_string();
    }
    String::new()
}

fn write_response(stream: &mut TcpStream, content_type: &str, body: &[u8]) -> Result<()> {
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

fn write_status(stream: &mut TcpStream, status: &str, body: &[u8]) -> Result<()> {
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

/// Non-blocking probe: bind, init Model, one frame, drop listener.
pub fn probe() -> Result<(SocketAddr, String)> {
    let (listener, handle) = bind_loft("127.0.0.1:0")?;
    let addr = handle.local_addr();
    let detail = format!("listen {}", handle.url());
    drop(listener);
    handle.request_stop();
    Ok((addr, detail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quit_key_stops_step_program() {
        let backend = WebBackend::new(COLS, ROWS);
        let mut program = StepProgram::with_backend(HelloTick::loft(), backend);
        program.init().expect("init");
        assert!(program.is_running());
        program
            .push_event(Event::Key(KeyEvent::new(KeyCode::Char('q'))))
            .expect("push q");
        let result = program.step().expect("step");
        assert!(!result.running);
        assert!(!program.is_running());
    }

    #[test]
    fn frame_contains_title_and_quit_hint() {
        let (listener, handle) = bind_loft("127.0.0.1:0").expect("bind");
        let text = handle.inner.lock().unwrap().frame_text.clone();
        drop(listener);
        assert!(
            text.contains("Loft") && text.to_ascii_lowercase().contains("quit"),
            "expected Loft + quit hint in frame, got:\n{text}"
        );
        handle.request_stop();
    }
}
