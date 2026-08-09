--- cargo-crates/v8-150.4.0/third_party/partition_alloc/src/partition_alloc/stack/stack.cc.orig	2026-06-10 02:22:19 UTC
+++ cargo-crates/v8-150.4.0/third_party/partition_alloc/src/partition_alloc/stack/stack.cc
@@ -18,6 +18,10 @@
 #include <pthread.h>
 #endif
 
+#if PA_BUILDFLAG(IS_FREEBSD)
+#include <pthread_np.h>
+#endif
+
 #if PA_BUILDFLAG(PA_LIBC_GLIBC)
 extern "C" void* __libc_stack_end;
 #endif
@@ -55,7 +59,11 @@ void* GetStackTop() {
 
 void* GetStackTop() {
   pthread_attr_t attr;
+#if PA_BUILDFLAG(IS_FREEBSD)
+  int error = pthread_attr_get_np(pthread_self(), &attr);
+#else
   int error = pthread_getattr_np(pthread_self(), &attr);
+#endif
   if (!error) {
     void* base;
     size_t size;
