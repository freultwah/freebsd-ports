--- codex-rs/app-server-daemon/src/update_loop_tests.rs.orig
+++ codex-rs/app-server-daemon/src/update_loop_tests.rs
@@ -194,7 +194,7 @@
     requested_urls: Mutex<Vec<String>>,
 }
 
-#[cfg(unix)]
+#[cfg(all(unix, not(target_os = "freebsd")))]
 #[tokio::test]
 async fn explicit_update_migrates_running_and_stopped_installations() {
     for (running, local) in [(false, false), (true, false), (false, true), (true, true)] {
@@ -952,7 +952,7 @@
     }
 }
 
-#[cfg(unix)]
+#[cfg(all(unix, not(target_os = "freebsd")))]
 #[tokio::test]
 async fn manual_update_restarts_local_daemon_with_automatic_updates_disabled() {
     check_manual_update_restart("app-server-daemon").await;
@@ -1239,7 +1239,7 @@
     assert!(error.to_string().contains("package root changed"));
 }
 
-#[cfg(unix)]
+#[cfg(all(unix, not(target_os = "freebsd")))]
 #[tokio::test]
 async fn daemon_owned_updates_require_and_request_an_isolated_installer() {
     let home = TempDir::new().unwrap();
@@ -1338,3 +1338,11 @@
         );
     }
 }
+
+#[cfg(target_os = "freebsd")]
+#[tokio::test]
+async fn upstream_updater_is_disabled_for_pkg_managed_codex() {
+    let home = TempDir::new().unwrap();
+    let (daemon, _) = manual_update_daemon(&home);
+    assert!(!super::manual_update::supported(&daemon).unwrap());
+}
