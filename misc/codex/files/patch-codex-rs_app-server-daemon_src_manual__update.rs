--- codex-rs/app-server-daemon/src/manual_update.rs.orig
+++ codex-rs/app-server-daemon/src/manual_update.rs
@@ -147,0 +148,4 @@
+    // FreeBSD binaries and dependencies are managed by pkg, not the installer.
+    if cfg!(target_os = "freebsd") {
+        return Ok(false);
+    }
@@ -160,0 +165,4 @@
+#[cfg(target_os = "freebsd")]
+const UNSUPPORTED_MESSAGE: &str = "Update Codex using pkg, then run `codex app-server daemon update --from-cli` to refresh the daemon package.";
+
+#[cfg(not(target_os = "freebsd"))]
