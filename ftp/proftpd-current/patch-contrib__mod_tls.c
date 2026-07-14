From 0000000000000000000000000000000000000000 Mon Sep 17 00:00:00 2001
From: Raivo Hool <raivo@lehma.com>
Date: Mon, 18 May 2026 23:59:00 +0300
Subject: [PATCH] mod_tls: fix tls_keyfile_check success handling for normal keys

`tls_keyfile_check()` currently initializes `ok` to FALSE and does not set
it to TRUE for successful unencrypted key loads. As a result,
`TLSECCertificateKeyFile`/`TLSRSACertificateKeyFile`/`TLSDSACertificateKeyFile`
can fail config parsing with an empty fatal message.

Initialize `ok` to TRUE and only flip it to FALSE on actual failures,
preserving the passphrase-protected key exception logic.
---
 contrib/mod_tls.c | 3 ++-
 1 file changed, 2 insertions(+), 1 deletion(-)

diff --git a/contrib/mod_tls.c b/contrib/mod_tls.c
index 018d9e6df..8f0f0f0f0 100644
--- contrib/mod_tls.c
+++ contrib/mod_tls.c
@@ -1044,7 +1044,7 @@ static int tls_pkcs11_check(pool *p, const char *text, char **errors,
 }
 
 static int tls_keyfile_check(pool *p, const char *path, char **errors) {
-  int res, ok = FALSE;
+  int res, ok = TRUE;
   SSL_CTX *ctx;
 
   ctx = SSL_CTX_new(SSLv23_server_method());
@@ -1062,6 +1062,7 @@ static int tls_keyfile_check(pool *p, const char *path, char **errors) {
     res = SSL_CTX_use_PrivateKey_file(ctx, path, X509_FILETYPE_PEM);
     if (res != 1) {
       unsigned long err_code;
+      ok = FALSE;
 
       err_code = ERR_peek_error();
       switch (ERR_GET_REASON(err_code)) {
-- 
2.47.0
