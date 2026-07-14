--- tests/src/lib.rs.orig	2026-04-27 07:26:39.869840000 +0000
+++ tests/src/lib.rs	2026-04-27 07:27:28.302884000 +0000
@@ -6,10 +6,10 @@
 
 #[cfg(test)]
 use ::store::registry::bootstrap::Bootstrap;
-#[cfg(not(target_env = "msvc"))]
+#[cfg(all(not(target_env = "msvc"), not(target_os = "freebsd")))]
 use jemallocator::Jemalloc;
 
-#[cfg(not(target_env = "msvc"))]
+#[cfg(all(not(target_env = "msvc"), not(target_os = "freebsd")))]
 #[global_allocator]
 static GLOBAL: Jemalloc = Jemalloc;
 
