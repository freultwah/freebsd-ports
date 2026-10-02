--- codex-rs/core/src/tools/runtimes/apply_patch.rs.orig
+++ codex-rs/core/src/tools/runtimes/apply_patch.rs
@@ -5,6 +5,7 @@
 //! sandboxing enforced by the explicit filesystem sandbox context.
 use crate::exec::is_likely_sandbox_denied;
 use crate::session::turn_context::TurnEnvironment;
+use crate::tools::handlers::apply_patch::apply_patch_file_system_sandbox;
 use crate::tools::sandboxing::Approvable;
 use crate::tools::sandboxing::ApprovalAction;
 use crate::tools::sandboxing::ExecApprovalRequirement;
@@ -174,6 +175,16 @@
         let started_at = Instant::now();
         let fs = req.turn_environment.environment.get_filesystem();
         let sandbox = Self::file_system_sandbox_context_for_attempt(req, attempt);
+        let sandbox = apply_patch_file_system_sandbox(&req.turn_environment, sandbox.as_ref());
+        // Keep local BSD symlink handling without changing remote executors
+        // or environments that restrict filesystem reads.
+        let local_bsd_full_read = cfg!(any(target_os = "freebsd", target_os = "openbsd"))
+            && !req.turn_environment.environment.is_remote()
+            && req
+                .turn_environment
+                .permission_profile_with_workspace_roots()
+                .file_system_sandbox_policy()
+                .has_full_disk_read_access();
         let mut stdout = Vec::new();
         let mut stderr = Vec::new();
         let result = codex_apply_patch::apply_patch_with_options(
@@ -182,7 +193,8 @@
                 update_file_mode: req.action.update_file_mode(),
                 // Only reject links when an otherwise-required sandbox was bypassed.
                 // Executor-managed sandboxes can have SandboxType::None.
-                follow_symlinks: attempt.sandbox_requested
+                follow_symlinks: local_bsd_full_read
+                    || attempt.sandbox_requested
                     || !attempt.manager.should_sandbox(
                         attempt.permissions,
                         self.sandbox_preference(),
@@ -193,7 +205,7 @@
             &mut stdout,
             &mut stderr,
             fs.as_ref(),
-            sandbox.as_ref(),
+            sandbox,
         )
         .await;
         let stdout = String::from_utf8_lossy(&stdout).into_owned();
