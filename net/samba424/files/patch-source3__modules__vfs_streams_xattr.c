--- source3/modules/vfs_streams_xattr.c.orig	2026-01-20 15:42:54.522818600 +0000
+++ source3/modules/vfs_streams_xattr.c	2026-03-20 10:24:12.438631000 +0000
@@ -158,7 +158,7 @@
 					value + valuelen,
 					fraglen);
 		if (ret < 0) {
-			if (errno == ENODATA) {
+			if (errno == ENOATTR) {
 				/*
 				 * Happens if fsetxattr_multi could not write
 				 * everything it intended to write. Return a
@@ -300,13 +300,13 @@
 			 */
 			ret = SMB_VFS_FREMOVEXATTR(fsp, xattr_name);
 
-			if ((ret == -1) && (errno == ENODATA)) {
+			if ((ret == -1) && (errno == ENOATTR)) {
 				/*
 				 * fsetxattr_multi writes an
 				 * uninterrupted sequence from 1 to
 				 * config->max_extents. If we can't
 				 * remove one, it might be because
-				 * it's not there (errno==ENODATA),
+				 * it's not there (errno==ENOATTR),
 				 * then nothing will follow.
 				 */
 				ret = 0;
@@ -396,7 +396,7 @@
 
 		TALLOC_FREE(xattr_name);
 		if (ret < 0) {
-			if (errno == ENODATA) {
+			if (errno == ENOATTR) {
 				return 0;
 			}
 			return ret;
