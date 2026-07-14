--- config.inc.php.orig	2026-02-11 20:53:07.000000000 +0000
+++ config.inc.php	2026-02-14 09:58:31.299661000 +0000
@@ -240,6 +240,10 @@
 if(@file_exists('/usr/bin/doveadm')) { // @ to silence openbase_dir stuff; see https://github.com/postfixadmin/postfixadmin/issues/171
     $CONF['dovecotpw'] = "/usr/bin/doveadm pw"; # debian
 }
+
+if(@file_exists('/usr/local/bin/doveadm')) {
+    $CONF['dovecotpw'] = "/usr/local/bin/doveadm pw"; # FreeBSD
+}  
 
 // Password validation
 // New/changed passwords will be validated using all regular expressions in the array.
