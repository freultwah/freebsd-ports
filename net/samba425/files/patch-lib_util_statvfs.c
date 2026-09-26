The BSD statfs/fstatfs wrappers pass struct statfs, not struct statvfs.

--- lib/util/statvfs.c.orig
+++ lib/util/statvfs.c
@@ -79,7 +79,7 @@
 
 #if defined(BSD_STYLE_STATVFS)
 
-static void bsd_init_statvfs(const struct statvfs *src,
+static void bsd_init_statvfs(const struct statfs *src,
 			     struct vfs_statvfs_struct *dst)
 {
 	dst->OptimalTransferSize = src->f_iosize;
