diff --git crates/common/src/network/listen.rs crates/common/src/network/listen.rs
--- crates/common/src/network/listen.rs
+++ crates/common/src/network/listen.rs
@@ -458,7 +458,7 @@ impl TcpListener {
     }
 
     fn is_bound_to(&self, local_addr: SocketAddr) -> bool {
-        local_addr.ip() == self.addr.ip()
+        (self.addr.ip().is_unspecified() || local_addr.ip() == self.addr.ip())
             && local_addr.port() != 0
             && (self.addr.port() == 0 || local_addr.port() == self.addr.port())
     }
