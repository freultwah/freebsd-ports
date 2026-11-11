--- cargo-crates/v8-152.2.0/build/modules/unified/modulemap_config.py.orig
+++ cargo-crates/v8-152.2.0/build/modules/unified/modulemap_config.py
@@ -63,7 +63,7 @@
 
 
 def headers(os):
-  is_linux = os == 'linux'
+  is_linux = os == 'linux' or os == 'openbsd' or os == 'freebsd'
   is_android = os == 'android'
   is_ios = os == 'ios'
   is_mac = os == 'mac'
