--- codex-rs/app-server-daemon/src/manual_update.rs.orig
+++ codex-rs/app-server-daemon/src/manual_update.rs
@@ -147,6 +147,10 @@
 }
 
 pub(super) fn supported(daemon: &Daemon) -> Result<bool> {
+    // FreeBSD binaries and dependencies are managed by pkg, not the installer.
+    if cfg!(target_os = "freebsd") {
+        return Ok(false);
+    }
     if daemon.is_stable_standalone_release()? {
         return Ok(true);
     }
@@ -160,6 +164,10 @@
         && std::fs::canonicalize(bin).is_ok_and(|bin| bin.starts_with(&release)))
 }
 
+#[cfg(target_os = "freebsd")]
+const UNSUPPORTED_MESSAGE: &str = "Update Codex using pkg, then run `codex app-server daemon update --from-cli` to refresh the daemon package.";
+
+#[cfg(not(target_os = "freebsd"))]
 const UNSUPPORTED_MESSAGE: &str =
     "This command requires a daemon package selected from its managed releases directory.";
 
