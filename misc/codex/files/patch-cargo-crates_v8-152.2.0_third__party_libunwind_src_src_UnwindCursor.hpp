--- cargo-crates/v8-152.2.0/third_party/libunwind/src/src/UnwindCursor.hpp.orig
+++ cargo-crates/v8-152.2.0/third_party/libunwind/src/src/UnwindCursor.hpp
@@ -113,6 +113,8 @@
 #endif
 
 namespace libunwind {
+
+static thread_local UnwindInfoSectionsCache uwis_cache;
 
 #if defined(_LIBUNWIND_SUPPORT_DWARF_UNWIND)
 /// Cache of recently found FDEs.
@@ -2939,7 +2941,14 @@
 
   // Ask address space object to find unwind sections for this pc.
   UnwindInfoSections sects;
-  if (_addressSpace.template findUnwindSections<R>(pc, sects)) {
+  bool have_sects = false;
+  if (uwis_cache.getUnwindInfoSectionsForPC(pc, sects))
+    have_sects = true;
+  else if (_addressSpace.template findUnwindSections<R>(pc, sects)) {
+    uwis_cache.setUnwindInfoSectionsForPC(pc, sects);
+    have_sects = true;
+  }
+  if (have_sects) {
 #if defined(_LIBUNWIND_SUPPORT_COMPACT_UNWIND)
     // If there is a compact unwind encoding table, look there first.
     if (sects.compact_unwind_section != 0) {
