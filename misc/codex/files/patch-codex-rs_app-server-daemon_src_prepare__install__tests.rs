--- codex-rs/app-server-daemon/src/prepare_install_tests.rs.orig
+++ codex-rs/app-server-daemon/src/prepare_install_tests.rs
@@ -385,0 +386,34 @@
+
+// Ports provide rg as a runtime dependency rather than copying it into Codex.
+#[cfg(target_os = "freebsd")]
+#[tokio::test]
+async fn freebsd_package_uses_system_ripgrep_and_never_runs_the_upstream_updater() {
+    let temp = tempfile::TempDir::new().expect("temp");
+    let source = temp.path().join("package");
+    let bin = package(&source, "0.157.0");
+    std::fs::remove_file(source.join("codex-path/rg")).expect("remove bundled rg");
+    let home = temp.path().join("home");
+    let daemon = daemon(&home);
+    prepare_from_package(
+        &daemon,
+        &DaemonSettings::default(),
+        InstallMode::Missing,
+        Some(&source),
+        &bin,
+        |_| Ok(true),
+    )
+    .await
+    .expect("seed FreeBSD package");
+    assert!(
+        !daemon
+            .is_stable_standalone_release()
+            .expect("updater eligibility")
+    );
+    assert!(
+        !daemon
+            .ensure_managed_updater(&DaemonSettings::default())
+            .await
+            .expect("disable the installer updater")
+    );
+    assert!(!daemon.update_pid_file.exists());
+}
