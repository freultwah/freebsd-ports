Include the socket address family definitions explicitly for FreeBSD.

--- src/tracker/tracker_worker.cc.orig
+++ src/tracker/tracker_worker.cc
@@ -2,6 +2,9 @@
 
 #include "tracker_worker.h"
 
+#ifdef __FreeBSD__
+#include <sys/socket.h>
+#endif
 #include <netinet/in.h>
 
 #include "torrent/exceptions.h"
