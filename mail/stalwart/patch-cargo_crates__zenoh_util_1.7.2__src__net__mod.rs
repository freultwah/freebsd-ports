--- ./cargo-crates/zenoh-util-1.7.2/src/net/mod.rs.orig	2025-09-16 14:27:06.887334000 +0000
+++ ./cargo-crates/zenoh-util-1.7.2/src/net/mod.rs	2025-09-16 14:27:34.995870000 +0000
@@ -447,13 +447,13 @@
     Ok(())
 }
 
-#[cfg(any(target_os = "macos", target_os = "windows"))]
+#[cfg(any(target_os = "macos", target_os = "windows", target_os = "freebsd"))]
 pub fn set_bind_to_device_tcp_socket(socket: &TcpSocket, iface: &str) -> ZResult<()> {
     tracing::warn!("Binding the socket {socket:?} to the interface {iface} is not supported on macOS and Windows");
     Ok(())
 }
 
-#[cfg(any(target_os = "macos", target_os = "windows"))]
+#[cfg(any(target_os = "macos", target_os = "windows", target_os = "freebsd"))]
 pub fn set_bind_to_device_udp_socket(socket: &UdpSocket, iface: &str) -> ZResult<()> {
     tracing::warn!("Binding the socket {socket:?} to the interface {iface} is not supported on macOS and Windows");
     Ok(())
