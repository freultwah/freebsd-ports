--- codex-rs/app-server-daemon/src/backend/pid.rs.orig
+++ codex-rs/app-server-daemon/src/backend/pid.rs
@@ -735 +735,6 @@
-    let output = Command::new("ps")
+    let mut command = Command::new("ps");
+    // FreeBSD uses this fallback for process identity. Keep timestamps stable
+    // when clients attach from terminals with different locale/timezone settings.
+    #[cfg(target_os = "freebsd")]
+    command.env("LC_ALL", "C").env("TZ", "UTC");
+    let output = command
