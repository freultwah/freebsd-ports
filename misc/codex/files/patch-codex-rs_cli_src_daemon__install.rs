--- codex-rs/cli/src/daemon_install.rs.orig
+++ codex-rs/cli/src/daemon_install.rs
@@ -24,0 +25,5 @@
+    let update_hint = if cfg!(target_os = "freebsd") {
+        "Use pkg for future updates, then run `codex app-server daemon update --from-cli` again."
+    } else {
+        "The selected package will be pinned. Run `codex app-server daemon update` to return to production updates."
+    };
@@ -26 +31 @@
-        "Replace installed daemon version {} with CLI version {} from {}.\nThe daemon package will be installed in {}.\nThe selected package will be pinned. Run `codex app-server daemon update` to return to production updates.",
+        "Replace installed daemon version {} with CLI version {} from {}.\nThe daemon package will be installed in {}.\n{update_hint}",
