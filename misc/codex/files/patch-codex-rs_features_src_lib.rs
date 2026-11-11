--- codex-rs/features/src/lib.rs.orig
+++ codex-rs/features/src/lib.rs
@@ -1100,7 +1100,8 @@
         id: Feature::DaemonAutoStart,
         key: "daemon_auto_start",
         stage: Stage::Stable,
-        default_enabled: true,
+        // Keep the pkg-managed FreeBSD daemon opt-in.
+        default_enabled: !cfg!(target_os = "freebsd"),
     },
     FeatureSpec {
         id: Feature::TuiMultiBackend,
