--- codex-rs/cli/src/daemon_install_tests.rs.orig
+++ codex-rs/cli/src/daemon_install_tests.rs
@@ -11,0 +12,8 @@
+    #[cfg(target_os = "freebsd")]
+    insta::assert_snapshot!(describe_install(&request), @r"
+    Replace installed daemon version 0.152.0 with CLI version 0.0.0 from /cli/package.
+    The daemon package will be installed in /home/packages/app-server-daemon.
+    Use pkg for future updates, then run `codex app-server daemon update --from-cli` again.
+    The running daemon will restart; active or queued work may be interrupted.
+    ");
+    #[cfg(not(target_os = "freebsd"))]
