--- common.php.orig	2026-03-14 15:28:57.871520000 +0000
+++ common.php	2026-03-14 15:28:36.114196000 +0000
@@ -1,11 +1,5 @@
 <?php
 
-if (!is_file(dirname(__FILE__) . '/vendor/autoload.php')) {
-    die("vendor/autoload.php is missing. Please run 'install.sh' or follow the instructions in INSTALL.md");
-}
-
-require_once(dirname(__FILE__) . '/vendor/autoload.php');
-
 /**
  * Postfix Admin
  *
@@ -65,6 +59,44 @@
 
 $incpath = dirname(__FILE__);
 
+/**
+ * @param string $class
+ * __autoload implementation, for use with spl_autoload_register().
+ */
+function postfixadmin_autoload($class)
+{
+    $base = __DIR__;
+
+    // 1) Smarty 5 uses PSR-4 classes rooted in lib/smarty/src/.
+    $smarty_prefix = 'Smarty\\';
+    if (strncmp($class, $smarty_prefix, strlen($smarty_prefix)) === 0) {
+        $relative = substr($class, strlen($smarty_prefix));
+        $file = $base . '/lib/smarty/src/' . str_replace('\\', '/', $relative) . '.php';
+        if (is_file($file)) {
+            require_once $file;
+            return;
+        }
+    }
+
+    // 2) Namespaced PostfixAdmin classes (e.g. model\Languages)
+    if (strpos($class, '\\') !== false) {
+        $file = $base . '/' . str_replace('\\', '/', $class) . '.php';
+        if (is_file($file)) {
+            require_once $file;
+            return;
+        }
+    }
+
+    // 3) Legacy global classes stored in model/
+    $legacy = $base . '/model/' . $class . '.php';
+    if (is_file($legacy)) {
+        require_once $legacy;
+        return;
+    }
+}
+
+spl_autoload_register('postfixadmin_autoload');
+
 if (!is_file("$incpath/config.inc.php")) {
     die("config.inc.php is missing!");
 }
@@ -107,6 +139,9 @@
     if (!isset($PALANG)) {
         die("environment not setup correctly");
     }
 }
 
+require_once(__DIR__  . '/lib/pacrypt/src/Crypt.php');
+require_once(__DIR__  . '/lib/pacrypt/src/Exception.php');
+
 /* vim: set expandtab softtabstop=4 tabstop=4 shiftwidth=4: */
