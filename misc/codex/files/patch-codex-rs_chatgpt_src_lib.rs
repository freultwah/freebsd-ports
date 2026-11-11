Raise the recursion limit for the async connectors::list_connectors layout.

--- codex-rs/chatgpt/src/lib.rs.orig
+++ codex-rs/chatgpt/src/lib.rs
@@ -1,3 +1,5 @@
+#![recursion_limit = "256"]
+
 #![recursion_limit = "256"]
 
 pub mod apply_command;
