--- cargo-crates/v8-152.2.0/v8/src/flags/flags.cc.orig
+++ cargo-crates/v8-152.2.0/v8/src/flags/flags.cc
@@ -18,6 +18,10 @@
 #include <string_view>
 #include <unordered_map>
 #include <vector>
+
+#if V8_OS_OPENBSD
+#include <sys/mman.h>
+#endif
 
 #include "src/base/fpu.h"
 #include "src/base/hashing.h"
@@ -978,6 +982,10 @@
   // Note that for string flags we only protect the pointer itself, but not the
   // string storage. TODO(12887): Fix this.
   base::OS::SetDataReadOnly(&v8_flags, sizeof(v8_flags));
+#if V8_OS_OPENBSD
+  if (mimmutable(&v8_flags, sizeof(v8_flags)) == -1)
+    FATAL("unable to set immutability of v8_flags");
+#endif
 }
 
 // static
