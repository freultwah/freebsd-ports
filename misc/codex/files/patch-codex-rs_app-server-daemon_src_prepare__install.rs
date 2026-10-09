--- codex-rs/app-server-daemon/src/prepare_install.rs.orig
+++ codex-rs/app-server-daemon/src/prepare_install.rs
@@ -448,6 +448,10 @@
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
@@ -477,6 +481,9 @@
 
 fn platform_target() -> Result<&'static str> {
     match (std::env::consts::OS, std::env::consts::ARCH) {
+        ("freebsd", "x86_64") => Ok("x86_64-unknown-freebsd"),
+        ("freebsd", "aarch64") => Ok("aarch64-unknown-freebsd"),
+        ("freebsd", "x86") => Ok("i686-unknown-freebsd"),
         ("macos", "aarch64") => Ok("aarch64-apple-darwin"),
         ("macos", "x86_64") => Ok("x86_64-apple-darwin"),
         ("linux", "aarch64") if cfg!(target_env = "gnu") => Ok("aarch64-unknown-linux-gnu"),
