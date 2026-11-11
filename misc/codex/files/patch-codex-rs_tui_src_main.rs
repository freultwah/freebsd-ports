--- codex-rs/tui/src/main.rs.orig
+++ codex-rs/tui/src/main.rs
@@ -1,5 +1,7 @@
 // Release builds exceed rustc's default query depth when laying out the async
 // MessageProcessor::handle_initialized_client_request future.
+#![recursion_limit = "256"]
+
 #![recursion_limit = "256"]
 
 use clap::Parser;
