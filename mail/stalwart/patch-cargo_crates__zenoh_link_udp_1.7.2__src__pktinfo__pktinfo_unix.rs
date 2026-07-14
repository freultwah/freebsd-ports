--- cargo-crates/zenoh-link-udp-1.7.2/src/pktinfo/pktinfo_unix.rs.orig	2025-09-16 14:47:32.031509000 +0000
+++ cargo-crates/zenoh-link-udp-1.7.2/src/pktinfo/pktinfo_unix.rs	2025-09-16 16:46:25.613296000 +0000
@@ -56,19 +56,44 @@
 
 pub(crate) fn enable_pktinfo(socket: &UdpSocket) -> io::Result<PktInfoRetrievalData> {
     let local_src_addr = socket.local_addr()?;
-    match local_src_addr.is_ipv6() {
-        false => unsafe {
-            setsockopt(socket.as_raw_fd(), libc::IPPROTO_IP, libc::IP_PKTINFO, 1)?;
-        },
-        true => unsafe {
+
+    #[cfg(target_os = "freebsd")]
+    unsafe {
+        if local_src_addr.is_ipv6() {
+            // IPv6: FreeBSD supports IPV6_RECVPKTINFO, same as Linux
             setsockopt(
                 socket.as_raw_fd(),
                 libc::IPPROTO_IPV6,
                 libc::IPV6_RECVPKTINFO,
                 1,
             )?;
-        },
+        } else {
+            // IPv4: Use IP_RECVDSTADDR instead of IP_PKTINFO
+            let enable: libc::c_int = 1;
+            libc::setsockopt(
+                socket.as_raw_fd(),
+                libc::IPPROTO_IP,
+                libc::IP_RECVDSTADDR,
+                &enable as *const _ as *const libc::c_void,
+                std::mem::size_of::<libc::c_int>() as libc::socklen_t,
+            );
+        }
     }
+
+    #[cfg(not(target_os = "freebsd"))]
+    unsafe {
+        if local_src_addr.is_ipv6() {
+            setsockopt(
+                socket.as_raw_fd(),
+                libc::IPPROTO_IPV6,
+                libc::IPV6_RECVPKTINFO,
+                1,
+            )?;
+        } else {
+            setsockopt(socket.as_raw_fd(), libc::IPPROTO_IP, libc::IP_PKTINFO, 1)?;
+        }
+    }
+
     Ok(PktInfoRetrievalData {
         port: local_src_addr.port(),
     })
@@ -79,10 +104,23 @@
     local_port: u16,
     buf: &mut [u8],
 ) -> io::Result<(usize, SocketAddr, Option<SocketAddr>)> {
+    use std::os::unix::io::AsRawFd;
+
     let mut addr_src: MaybeUninit<libc::sockaddr_storage> = MaybeUninit::uninit();
     let mut msg_iov = IoSliceMut::new(buf);
+
+    // Allocate control message buffer depending on platform
+    #[cfg(target_os = "freebsd")]
     let mut cmsg = {
         let space = unsafe {
+            libc::CMSG_SPACE(mem::size_of::<libc::in_addr>() as libc::c_uint) as usize
+        };
+        Vec::<u8>::with_capacity(space)
+    };
+
+    #[cfg(not(target_os = "freebsd"))]
+    let mut cmsg = {
+        let space = unsafe {
             libc::CMSG_SPACE(mem::size_of::<libc::in_pktinfo>() as libc::c_uint) as usize
         };
         Vec::<u8>::with_capacity(space)
@@ -135,6 +173,8 @@
         let p = unsafe { libc::CMSG_DATA(h) };
 
         match (h.cmsg_level, h.cmsg_type) {
+            // ---- Linux / Android: use in_pktinfo ----
+            #[cfg(any(target_os = "linux", target_os = "android"))]
             (libc::IPPROTO_IP, libc::IP_PKTINFO) => {
                 let pktinfo = unsafe { ptr::read_unaligned(p as *const libc::in_pktinfo) };
                 addr_dst = Some(SocketAddr::new(
@@ -142,6 +182,18 @@
                     local_port,
                 ));
             }
+
+            // ---- FreeBSD: use IP_RECVDSTADDR and in_addr ----
+            #[cfg(target_os = "freebsd")]
+            (libc::IPPROTO_IP, libc::IP_RECVDSTADDR) => {
+                let inaddr = unsafe { ptr::read_unaligned(p as *const libc::in_addr) };
+                addr_dst = Some(SocketAddr::new(
+                    IpAddr::V4(Ipv4Addr::from(u32::from_be(inaddr.s_addr))),
+                    local_port,
+                ));
+            }
+
+            // ---- IPv6 (same on Linux/FreeBSD) ----
             (libc::IPPROTO_IPV6, libc::IPV6_PKTINFO) => {
                 let pktinfo = unsafe { ptr::read_unaligned(p as *const libc::in6_pktinfo) };
                 addr_dst = Some(SocketAddr::new(
@@ -149,6 +201,7 @@
                     local_port,
                 ));
             }
+
             _ => {
                 header = unsafe {
                     let p = libc::CMSG_NXTHDR(&mhdr as *const _, h as *const _);
@@ -157,6 +210,7 @@
             }
         }
     }
+
     Ok((bytes_recv as _, addr_src, addr_dst))
 }
 
