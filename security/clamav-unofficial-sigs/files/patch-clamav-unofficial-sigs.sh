--- clamav-unofficial-sigs.sh.orig
+++ clamav-unofficial-sigs.sh
@@ -1704,20 +1704,14 @@
 enable_log="no"
 custom_config="no"
 we_have_a_config="0"
-
-
-# Attempt to scan for a valid config dir
-# NOTE: a missing config dir is only fatal after command line parsing,
-# so that -c/--config can still supply a custom config on fresh systems
-if [ -f "/etc/clamav-unofficial-sigs/master.conf" ] ; then
-  config_dir="/etc/clamav-unofficial-sigs"
-elif [ -f "/usr/local/etc/clamav-unofficial-sigs/master.conf" ] ; then
-  config_dir="/usr/local/etc/clamav-unofficial-sigs/"
-elif [ -f "/opt/homebrew/etc/clamav-unofficial-sigs/master.conf" ] ; then
-  # macOS Apple Silicon homebrew
-  config_dir="/opt/homebrew/etc/clamav-unofficial-sigs/"
-elif [ -f "/opt/zimbra/conf/clamav-unofficial-sigs/master.conf" ] ; then
-  config_dir="/opt/zimbra/conf/clamav-unofficial-sigs/"
+pkg_mgr="FreeBSD ports"
+pkg_rm="pkg delete"
+
+
+# Use the configuration installed by the FreeBSD port.
+# Leave custom configuration handling to -c/--config below.
+if [ -f "%%PREFIX%%/etc/clamav-unofficial-sigs/master.conf" ] ; then
+  config_dir="%%PREFIX%%/etc/clamav-unofficial-sigs"
 else
   config_dir=""
 fi
@@ -1731,21 +1725,6 @@
   fi
   if [ -r "${config_dir}/os.conf" ] ; then
       config_files+=( "${config_dir}/os.conf" )
-  else
-      #find the a suitable os.*.conf file
-      os_config_number=$(find "$config_dir" -type f -iname "os.*.conf" | wc -l)
-      if [ "$os_config_number" == "0" ] ; then
-          xshok_pretty_echo_and_log "WARNING: no os.conf or os.*.conf found"
-      elif [ "$os_config_number" == "1" ] ; then
-          config_file="$(find "$config_dir" -type f -iname "os.*.conf" | head -n1)"
-          if [ -r "${config_file}" ]; then
-              config_files+=( "${config_file}" )
-          else
-              xshok_pretty_echo_and_log "WARNING: ${config_file} is not readable"
-          fi
-      else
-          xshok_pretty_echo_and_log "WARNING: Too many os.*.conf configs found"
-      fi
   fi
   if [ -r "${config_dir}/user.conf" ] ; then
       config_files+=( "${config_dir}/user.conf" )
@@ -1878,10 +1857,8 @@
         else
             xshok_pretty_echo_and_log "WARNING: ${config_dir}/master.conf not found"
         fi
-        #find the a suitable os.conf or os.*.conf file
-        config_file="$(find "$config_dir" -type f -iname "os.conf" -o -iname "os.*.conf" | tail -n1)"
-        if [ -r "${config_file}" ] ; then
-            config_files+=( "${config_file}" )
+        if [ -r "${config_dir}/os.conf" ] ; then
+            config_files+=( "${config_dir}/os.conf" )
         else
             xshok_pretty_echo_and_log "WARNING: ${config_dir}/os.conf not found"
         fi
