--- common.php.orig	2024-03-08 13:46:14.000000000 +0200
+++ common.php	2024-03-22 14:39:44.954841000 +0200
@@ -1,7 +1,4 @@
 <?php
-
-require_once(dirname(__FILE__) . '/vendor/autoload.php');
-
 /**
  * Postfix Admin
  *
@@ -47,6 +44,22 @@
 
 $incpath = dirname(__FILE__);
 
+/**
+ * @param string $class
+ * __autoload implementation, for use with spl_autoload_register().
+ */
+function postfixadmin_autoload($class) {
+    $PATH = dirname(__FILE__) . '/model/' . $class . '.php';
+
+    if (is_file($PATH)) {
+        require_once($PATH);
+        return true;
+    }
+    return false;
+}
+
+spl_autoload_register('postfixadmin_autoload');
+
 if (!is_file("$incpath/config.inc.php")) {
     die("config.inc.php is missing!");
 }
@@ -90,7 +103,12 @@
     if (!isset($PALANG)) {
         die("environment not setup correctly");
     }
+    require_once(__DIR__  . '/lib/smarty/libs/Autoloader.php');
     Smarty_Autoloader::register();
 }
+
+require_once(__DIR__  . '/lib/pacrypt/src/Crypt.php');
+require_once(__DIR__  . '/lib/pacrypt/src/Exception.php');
+
 
 /* vim: set expandtab softtabstop=4 tabstop=4 shiftwidth=4: */
