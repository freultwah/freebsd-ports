--- src/mdns_avahi.c.orig	2023-10-14 02:58:52.732056000 +0300
+++ src/mdns_avahi.c	2023-10-14 03:00:53.497801000 +0300
@@ -36,6 +36,7 @@
 #include <net/if.h>
 #include <unistd.h>
 #include <fcntl.h>
+#include <poll.h>
 
 #include <event2/event.h>
 
@@ -582,8 +583,7 @@
   struct addrinfo *ai;
   char strport[32];
   int sock;
-  fd_set fdset;
-  struct timeval timeout = { MDNS_CONNECT_TEST_TIMEOUT, 0 };
+  struct pollfd fd;
   socklen_t len;
   int flags;
   int error;
@@ -639,10 +639,11 @@
   // the case, but FreeBSD connect() sometimes returns immediate success.
   if (ret != 0)
     {
-      FD_ZERO(&fdset);
-      FD_SET(sock, &fdset);
+      // Use poll here since select requires using fdset that would be overflowed in FreeBSD
+      fd.fd = sock;
+      fd.events = POLLOUT;
 
-      ret = select(sock + 1, NULL, &fdset, NULL, &timeout);
+      ret = poll(&fd, 1, MDNS_CONNECT_TEST_TIMEOUT * 1000);
       if (ret < 0)
 	{
 	  DPRINTF(E_WARN, L_MDNS, "Connection test to %s:%d failed with select error: %s\n", address_log, port, strerror(errno));
