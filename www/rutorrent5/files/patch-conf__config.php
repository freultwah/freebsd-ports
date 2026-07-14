--- conf/config.php.orig	2025-04-28 15:00:16.000000000 +0000
+++ conf/config.php	2025-05-02 07:03:43.805088000 +0000
@@ -30,14 +30,14 @@
 
 	$log_file = $_ENV['RU_LOG_FILE'] ?? '/tmp/errors.log'; // path to log file (comment or leave blank to disable logging)
 
-	$saveUploadedTorrents = true;		// Save uploaded torrents to profile/torrents directory or not
+	$saveUploadedTorrents = false;		// Save uploaded torrents to profile/torrents directory or not
 	$overwriteUploadedTorrents = false;	// Overwrite existing uploaded torrents in profile/torrents directory or make unique name
 
 	$topDirectory = $_ENV['RU_TOP_DIR'] ?? '/';			// Upper available directory. Absolute path with trail slash.
 	$forbidUserSettings = false;
 
-	$scgi_port = $_ENV['RU_SCGI_PORT'] ?? 5000;
-	$scgi_host = $_ENV['RU_SCGI_HOST'] ?? "127.0.0.1";
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
