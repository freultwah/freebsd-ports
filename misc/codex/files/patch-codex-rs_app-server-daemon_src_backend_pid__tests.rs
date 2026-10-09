--- codex-rs/app-server-daemon/src/backend/pid_tests.rs.orig
+++ codex-rs/app-server-daemon/src/backend/pid_tests.rs
@@ -650,6 +650,7 @@
     );
 }
 
+#[cfg(not(target_os = "freebsd"))]
 #[tokio::test]
 async fn stale_creation_time_never_stops_reused_pid() {
     let temp = TempDir::new().expect("temp");
@@ -697,6 +698,35 @@
         assert!(!backend.pid_file.exists());
         assert!(!backend.pid_file.with_extension("shutdown").exists());
     }
+}
+
+#[cfg(target_os = "freebsd")]
+#[tokio::test]
+async fn stale_creation_time_never_stops_reused_pid() {
+    let temp = TempDir::new().expect("temp dir");
+    let backend = PidBackend::new(
+        temp.path().join("codex"),
+        temp.path().join("server.pid"),
+        /*remote_control_enabled*/ false,
+    );
+    let record = PidRecord {
+        pid: std::process::id(),
+        process_start_time: "stale".into(),
+        process_identity: None,
+        executable_identity: None,
+    };
+    std::fs::write(&backend.pid_file, serde_json::to_vec(&record).unwrap()).unwrap();
+    let error = backend
+        .stop()
+        .await
+        .expect_err("refuse mismatched live PID");
+    assert!(
+        error
+            .to_string()
+            .contains("cannot verify pid-managed process")
+    );
+    assert!(backend.pid_file.exists());
+    assert!(super::process_exists(std::process::id()));
 }
 
 #[cfg(windows)]
