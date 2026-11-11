--- codex-rs/app-server-daemon/src/backend/pid.rs.orig
+++ codex-rs/app-server-daemon/src/backend/pid.rs
@@ -816,7 +816,12 @@
 
 #[cfg(unix)]
 async fn read_process_details(pid: u32) -> Result<(String, String)> {
-    let output = Command::new("ps")
+    let mut command = Command::new("ps");
+    // FreeBSD uses this fallback for process identity. Keep timestamps stable
+    // when clients attach from terminals with different locale/timezone settings.
+    #[cfg(target_os = "freebsd")]
+    command.env("LC_ALL", "C").env("TZ", "UTC");
+    let output = command
         .args(["-p", &pid.to_string(), "-o", "stat=", "-o", "lstart="])
         .kill_on_drop(true)
         .output()
