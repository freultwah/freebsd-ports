--- conf/config.php.orig	2023-05-02 01:51:40.000000000 +0300
+++ conf/config.php	2023-05-02 10:51:35.588054000 +0300
@@ -28,16 +28,16 @@
 	$do_diagnostic = true;			// Diagnose ruTorrent. Recommended to keep enabled, unless otherwise required.
 	$al_diagnostic = true;			// Diagnose auto-loader. Set to "false" to make composer plugins work.
 
-	$log_file = '/tmp/errors.log';		// path to log file (comment or leave blank to disable logging)
+	$log_file = '/usr/local/www/rutorrent/tmp/errors.log';		// path to log file (comment or leave blank to disable logging)
 
-	$saveUploadedTorrents = true;		// Save uploaded torrents to profile/torrents directory or not
+	$saveUploadedTorrents = false;		// Save uploaded torrents to profile/torrents directory or not
 	$overwriteUploadedTorrents = false;	// Overwrite existing uploaded torrents in profile/torrents directory or make unique name
 
 	$topDirectory = '/';			// Upper available directory. Absolute path with trail slash.
 	$forbidUserSettings = false;
 
-	$scgi_port = 5000;
-	$scgi_host = "127.0.0.1";
+	$scgi_port = 0;
+	$scgi_host = "unix:///server/Bittorrent/socket/rtorrent.socket";
 
 	// For web->rtorrent link through unix domain socket
 	// (scgi_local in rtorrent conf file), change variables
@@ -46,17 +46,24 @@
 	// $scgi_port = 0;
 	// $scgi_host = "unix:///tmp/rpc.socket";
 
-	$XMLRPCMountPoint = "/RPC2";		// DO NOT DELETE THIS LINE!!! DO NOT COMMENT THIS LINE!!!
+	$XMLRPCMountPoint = "/rutorrent/RPC2";		// DO NOT DELETE THIS LINE!!! DO NOT COMMENT THIS LINE!!!
 
 	$throttleMaxSpeed = 327625*1024;	// DO NOT EDIT THIS LINE!!! DO NOT COMMENT THIS LINE!!!
 	// Can't be greater then 327625*1024 due to limitation in libtorrent ResourceManager::set_max_upload_unchoked function.
 
 	$pathToExternals = array(
-		"php"	=> '',			// Something like /usr/bin/php. If empty, will be found in PATH.
-		"curl"	=> '',			// Something like /usr/bin/curl. If empty, will be found in PATH.
-		"gzip"	=> '',			// Something like /usr/bin/gzip. If empty, will be found in PATH.
-		"id"	=> '',			// Something like /usr/bin/id. If empty, will be found in PATH.
-		"stat"	=> '',			// Something like /usr/bin/stat. If empty, will be found in PATH.
+		"php"   => '/usr/local/bin/php',
+		"curl"  => '/usr/local/bin/curl',
+		"gzip"  => '/usr/bin/gzip',
+		"id"    => '/usr/bin/id',
+		"stat"  => '/usr/bin/stat',
+		"pgrep" => '/usr/bin/pgrep',
+		"unzip" => '/usr/bin/unzip',
+		"unrar" => '/usr/local/bin/unrar',
+		"ffmpeg"=> '/usr/local/bin/ffmpeg',
+		"sox"   => '/usr/local/bin/sox',
+		"mediainfo"=> '/usr/local/bin/mediainfo',
+		"python" => '/usr/local/bin/python3',
 	);
 
 	$localHostedMode = false;		// Set to true if rTorrent is hosted on the SAME machine as ruTorrent
@@ -74,7 +81,7 @@
 						// Both Webserver and rtorrent users must have read-write access to it.
 						// For example, if Webserver and rtorrent users are in the same group then the value may be 0770.
 
-	$tempDirectory = null;			// Temp directory. Absolute path with trail slash. If null, then autodetect will be used.
+	$tempDirectory = '/usr/local/www/rutorrent4/tmp/';		// Temp directory. Absolute path with trail slash. If null, then autodetect will be used.
 
 	$canUseXSendFile = false;		// If true then use X-Sendfile feature if it exist
 
