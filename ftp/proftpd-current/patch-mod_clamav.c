--- vendor/github.com/jbenden/mod_clamav/mod_clamav.c.orig	2024-01-26 15:21:15.960582000 +0200
+++ vendor/github.com/jbenden/mod_clamav/mod_clamav.c	2024-01-26 15:21:35.606000000 +0200
@@ -76,6 +76,8 @@
     return -1;
   }
 
+  int fxerrno = 0; // Add this line to declare and initialize fxerrno
+
   memset(buff, '\0', sizeof(buff));
   // Try to read in a loop in case fgets gets interrupted
   do {
