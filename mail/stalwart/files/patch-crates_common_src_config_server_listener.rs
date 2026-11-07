--- crates/common/src/config/server/listener.rs.orig
+++ crates/common/src/config/server/listener.rs
@@ -70,14 +70,32 @@
                 }
             }
         } else {
+            let id = ObjectId::new(ObjectType::NetworkListener, Id::singleton());
+            let bind_ip = match std::env::var("STALWART_RECOVERY_MODE_BIND") {
+                Ok(value) => match value.parse::<IpAddr>() {
+                    Ok(ip) => ip,
+                    Err(err) => {
+                        bp.build_error(
+                            id,
+                            format!("Invalid STALWART_RECOVERY_MODE_BIND: {err}"),
+                        );
+                        return servers;
+                    }
+                },
+                Err(std::env::VarError::NotPresent) => std::net::Ipv6Addr::UNSPECIFIED.into(),
+                Err(err) => {
+                    bp.build_error(id, format!("Invalid STALWART_RECOVERY_MODE_BIND: {err}"));
+                    return servers;
+                }
+            };
+            let bind_addr = StdSocketAddr::new(bind_ip, recovery_mode_port());
             servers.parse_server(
                 bp,
                 RegistryObject {
-                    id: ObjectId::new(ObjectType::NetworkListener, Id::singleton()),
+                    id,
                     object: NetworkListener {
                         bind: Map::new(vec![
-                            SocketAddr::from_str(&format!("[::]:{}", recovery_mode_port()))
-                                .unwrap(),
+                            SocketAddr::from_str(&bind_addr.to_string()).unwrap(),
                         ]),
                         name: "http-recovery".to_string(),
                         protocol: NetworkListenerProtocol::Http,
