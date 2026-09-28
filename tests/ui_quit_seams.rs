//! Quit / host seams for Croft+Loft (no GUI required in CI).

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use ftui::core::event::{Event, KeyCode, KeyEvent};
use ftui_web::step_program::StepProgram;
use ftui_web::WebBackend;
use mcload::ui::loft_host;
use mcload::ui::model::HelloTick;

#[test]
fn shared_model_quit_via_step_program() {
    let backend = WebBackend::new(80, 24);
    let mut program = StepProgram::with_backend(HelloTick::loft(), backend);
    program.init().expect("init loft model");
    assert_eq!(program.model().title(), "Loft");

    program
        .push_event(Event::Key(KeyEvent::new(KeyCode::Char('q'))))
        .expect("push q");
    let result = program.step().expect("step");
    assert!(!result.running, "q must stop StepProgram (Loft host seam)");
}

#[test]
fn croft_model_title_differs_only_by_name() {
    assert_eq!(HelloTick::croft().title(), "Croft");
    assert_eq!(HelloTick::loft().title(), "Loft");
}

#[test]
fn loft_host_http_key_q_stops_loop() {
    let (listener, handle) = loft_host::bind_loft("127.0.0.1:0").expect("bind");
    let addr = handle.local_addr();
    let url_host = format!("{addr}");

    let join = std::thread::spawn(move || loft_host::run_loop(listener, handle.clone()));

    // Give accept loop a moment, then POST q.
    std::thread::sleep(Duration::from_millis(50));
    let mut stream = TcpStream::connect(addr).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .ok();
    let req = format!(
        "POST /key HTTP/1.1\r\nHost: {url_host}\r\nContent-Length: 1\r\nConnection: close\r\n\r\nq"
    );
    stream.write_all(req.as_bytes()).expect("write");
    let mut resp = String::new();
    let _ = stream.read_to_string(&mut resp);

    join.join()
        .expect("host thread")
        .expect("run_loop should Ok after q");
}

#[test]
fn loft_host_frame_is_model_backed_not_static_stub() {
    let (listener, handle) = loft_host::bind_loft("127.0.0.1:0").expect("bind");
    let addr = handle.local_addr();
    let handle_loop = handle.clone();
    let join = std::thread::spawn(move || loft_host::run_loop(listener, handle_loop));

    std::thread::sleep(Duration::from_millis(30));
    let mut stream = TcpStream::connect(addr).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .ok();
    let req = format!("GET /frame HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
    stream.write_all(req.as_bytes()).expect("write");
    let mut resp = Vec::new();
    stream.read_to_end(&mut resp).ok();
    let text = String::from_utf8_lossy(&resp);
    assert!(
        text.contains("Loft") && text.to_ascii_lowercase().contains("quit"),
        "frame must come from shared Model (Loft + quit), got:\n{text}"
    );
    assert!(
        !text.contains("FrankenTUI Web is running."),
        "must not be the old static HTML stub body"
    );

    handle.request_stop();
    let _ = join.join();
}
