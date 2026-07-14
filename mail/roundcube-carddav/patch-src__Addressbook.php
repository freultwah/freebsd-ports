--- src/Addressbook.php.orig	2025-07-20 21:02:23.109031000 +0300
+++ src/Addressbook.php	2025-07-20 21:02:53.263790000 +0300
@@ -448,7 +448,7 @@
      * @param ?string $sort_order Sort order
      */
     // phpcs:ignore PSR1.Methods.CamelCapsMethodName -- method name defined by rcube_addressbook class
-    public function set_sort_order($sort_col, $sort_order = null): void
+    public function set_sort_order($sort_col = null, $sort_order = null)
     {
         if (isset($sort_col) && key_exists($sort_col, $this->coltypes)) {
             $this->sort_col = $sort_col;
