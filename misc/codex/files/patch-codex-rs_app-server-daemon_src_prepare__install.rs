--- codex-rs/app-server-daemon/src/prepare_install.rs.orig
+++ codex-rs/app-server-daemon/src/prepare_install.rs
@@ -74,7 +74,12 @@
             .ok()
             .map(|info| info.app_server_version),
         managed_codex_path,
-        message: "The CLI package is selected and pinned. Run `codex app-server daemon update` to return to production updates.".to_string(),
+        message: if cfg!(target_os = "freebsd") {
+            "The CLI package is selected and pinned. Use pkg for future updates, then run `codex app-server daemon update --from-cli` again."
+        } else {
+            "The CLI package is selected and pinned. Run `codex app-server daemon update` to return to production updates."
+        }
+        .to_string(),
     }))
 }
 
@@ -521,6 +526,10 @@
             "codex-path/rg"
         },
     ];
+    // The FreeBSD port uses the ripgrep package through PATH.
+    if cfg!(target_os = "freebsd") {
+        names.retain(|name| *name != "codex-path/rg");
+    }
     if cfg!(windows) {
         names.extend([
             "codex-resources/codex-command-runner.exe",
@@ -550,6 +559,9 @@
 
 fn platform_target() -> Result<&'static str> {
     match (std::env::consts::OS, std::env::consts::ARCH) {
+        ("freebsd", "x86_64") => Ok("x86_64-unknown-freebsd"),
+        ("freebsd", "aarch64") => Ok("aarch64-unknown-freebsd"),
+        ("freebsd", "x86") => Ok("i686-unknown-freebsd"),
         ("macos", "aarch64") => Ok("aarch64-apple-darwin"),
         ("macos", "x86_64") => Ok("x86_64-apple-darwin"),
         ("linux", "aarch64") if cfg!(target_env = "gnu") => Ok("aarch64-unknown-linux-gnu"),
