Convert Tokio's u32 PID to the platform's waitid id_t (i64 on FreeBSD).

--- codex-rs/exec-server/src/shell_snapshot_process.rs.orig
+++ codex-rs/exec-server/src/shell_snapshot_process.rs
@@ -51,7 +51,7 @@
                 if unsafe {
                     libc::waitid(
                         libc::P_PID,
-                        pid,
+                        pid.into(),
                         info.as_mut_ptr(),
                         libc::WEXITED | libc::WNOHANG | libc::WNOWAIT,
                     )
