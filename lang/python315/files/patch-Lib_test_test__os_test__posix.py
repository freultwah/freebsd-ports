See https://github.com/python/cpython/issues/148841 (independently),
see https://github.com/python/cpython/pull/148844#issuecomment-4406791031


--- Lib/test/test_os/test_posix.py.orig	2026-10-27 12:40:35 UTC
+++ Lib/test/test_os/test_posix.py
@@ -420,7 +420,7 @@ class PosixTester(unittest.TestCase):
             # so skip Solaris-based since they are likely to have ZFS.
             # issue33655: Also ignore EINVAL on *BSD since ZFS is also
             # often used there.
-            if inst.errno == errno.EINVAL and sys.platform.startswith(
+            if inst.errno in (errno.EINVAL, errno.EOPNOTSUPP) and sys.platform.startswith(
                 ('sunos', 'freebsd', 'openbsd', 'gnukfreebsd')):
                 raise unittest.SkipTest("test may fail on ZFS filesystems")
             elif inst.errno == errno.EOPNOTSUPP:
