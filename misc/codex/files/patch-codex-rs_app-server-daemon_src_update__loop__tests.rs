--- codex-rs/app-server-daemon/src/update_loop_tests.rs.orig
+++ codex-rs/app-server-daemon/src/update_loop_tests.rs
@@ -57 +57 @@
-#[cfg(unix)]
+#[cfg(all(unix, not(target_os = "freebsd")))]
@@ -688 +688 @@
-#[cfg(unix)]
+#[cfg(all(unix, not(target_os = "freebsd")))]
@@ -937 +937 @@
-#[cfg(unix)]
+#[cfg(all(unix, not(target_os = "freebsd")))]
@@ -1035,0 +1036,8 @@
+
+#[cfg(target_os = "freebsd")]
+#[tokio::test]
+async fn upstream_updater_is_disabled_for_pkg_managed_codex() {
+    let home = TempDir::new().unwrap();
+    let (daemon, _) = manual_update_daemon(&home);
+    assert!(!super::manual_update::supported(&daemon).unwrap());
+}
