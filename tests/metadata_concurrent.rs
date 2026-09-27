//! WS-4 — MetadataStore concurrent access contract.
//!
//! Two (or more) tasks read/write without panic/corruption; last-write-wins.

use mcload::metadata::{ConcurrentMemoryStore, MetadataStore, SnapshotRecord};
use std::sync::Arc;
use std::thread;

#[test]
fn concurrent_put_get_without_panic_or_corruption() {
    let store = Arc::new(ConcurrentMemoryStore::new());
    let threads = 8;
    let ops = 100;

    let mut handles = Vec::with_capacity(threads);
    for t in 0..threads {
        let store = Arc::clone(&store);
        handles.push(thread::spawn(move || {
            for i in 0..ops {
                let id = format!("k-{}", i % 10);
                let record = SnapshotRecord {
                    id: id.clone(),
                    path: format!("/t{t}/n{i}"),
                    payload: format!("v-{t}-{i}"),
                };
                store.put(record).expect("put");
                let _ = store.get(&id).expect("get");
            }
        }));
    }

    for h in handles {
        h.join().expect("thread should not panic");
    }

    let listed = store.list().expect("list");
    assert!(
        !listed.is_empty(),
        "store should retain records after concurrent writes"
    );
    assert!(
        listed.len() <= 10,
        "at most 10 distinct keys (k-0..k-9); got {}",
        listed.len()
    );

    for rec in &listed {
        let got = store
            .get(&rec.id)
            .expect("get after concurrent")
            .expect("key present");
        assert_eq!(got.id, rec.id);
        assert!(
            !got.payload.is_empty(),
            "payload must not be corrupted/empty"
        );
    }
}

#[test]
fn last_write_wins_for_same_key() {
    let store = ConcurrentMemoryStore::new();
    store
        .put(SnapshotRecord {
            id: "same".into(),
            path: "/a".into(),
            payload: "first".into(),
        })
        .expect("put first");
    store
        .put(SnapshotRecord {
            id: "same".into(),
            path: "/b".into(),
            payload: "second".into(),
        })
        .expect("put second");

    let got = store.get("same").expect("get").expect("present");
    assert_eq!(got.payload, "second");
    assert_eq!(got.path, "/b");
}
