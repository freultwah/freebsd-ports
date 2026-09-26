--- codex-rs/app-server-daemon/src/prepare_install.rs.orig
+++ codex-rs/app-server-daemon/src/prepare_install.rs
@@ -445,0 +446,4 @@
+    // The FreeBSD port uses the ripgrep package through PATH.
+    if cfg!(target_os = "freebsd") {
+        names.retain(|name| *name != "codex-path/rg");
+    }
@@ -474,0 +479,3 @@
+        ("freebsd", "x86_64") => Ok("x86_64-unknown-freebsd"),
+        ("freebsd", "aarch64") => Ok("aarch64-unknown-freebsd"),
+        ("freebsd", "x86") => Ok("i686-unknown-freebsd"),
