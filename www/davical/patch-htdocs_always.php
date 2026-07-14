--- htdocs/always.php.orig	2024-06-06 14:27:39.833685000 +0300
+++ htdocs/always.php	2024-06-06 14:28:00.323874000 +0300
@@ -141,6 +141,8 @@
 $c->base_directory = preg_replace('#/[^/]*$#', '', $_SERVER['DOCUMENT_ROOT']);
 $c->default_privileges = array('read-free-busy', 'schedule-deliver');
 $c->list_everyone = true;
+$c->external_refresh = 60;
+$c->scheduling_dkim_domain = false;
 
 $c->enable_auto_schedule = true;
 
@@ -159,13 +161,16 @@
 $c->hide_TODO = true;                      // VTODO only visible to collection owner
 $c->readonly_webdav_collections = true;    // WebDAV access is readonly
 
-// Kind of private configuration values
-$c->total_query_time = 0;
-
 // Any many times GetMoreInstances in inc/RRule.php should loop trying to
 // find more instances.
 $c->rrule_loop_limit = 100;
 
+// Kind of private configuration values
+$c->total_query_time = 0;
+
+// Are we in test mode?
+$c->test_mode = false;
+
 $c->dbg = array();
 
 
@@ -279,7 +284,7 @@
 else if ( isset($c->dbg['script_start']) && $c->dbg['script_start'] ) {
   // Only log this if more than a little debugging of some sort is turned on, somewhere
   @dbg_error_log( 'LOG', '==========> method =%s= =%s= =%s= =%s= =%s=',
-         $_SERVER['REQUEST_METHOD'], $c->protocol_server_port_script, (isset($_SERVER['PATH_INFO']) ? $_SERVER['PATH_INFO'] : '$_SERVER[PATH_INFO] undefined'), $c->base_url, $c->base_directory );
+         $_SERVER['REQUEST_METHOD'], $c->protocol_server_port_script, (isset($_SERVER['PATH_INFO']) && $_SERVER['PATH_INFO'] !== '' ? $_SERVER['PATH_INFO'] : '$_SERVER[PATH_INFO] undefined'), $c->base_url, $c->base_directory );
 }
 
 /**
@@ -297,7 +302,7 @@
 *
 */
 $c->code_version = 0;
-$c->want_awl_version = '0.64';
+$c->want_awl_version = '0.65';
 $c->version_string = '1.1.12'; // The actual version # is replaced into that during the build /release process
 if ( isset($c->version_string) && preg_match( '/(\d+)\.(\d+)\.(\d+)(.*)/', $c->version_string, $matches) ) {
   $c->code_major = $matches[1];
@@ -308,6 +313,12 @@
   @header( sprintf('Server: %d.%d', $c->code_major, $c->code_minor) );
 }
 
+// What is our minimum supported version of PHP?
+$c->minimum_php_version = '5.4.0';
+
+// Default UserAgent string when fetching content from elsewhere.
+$c->external_ua_string = "DAViCal/" . $c->version_string;
+
 /**
 * Force the domain name to what was in the configuration file
 */
@@ -315,7 +326,7 @@
 
 require_once('AwlQuery.php');
 
-$c->want_dbversion = array(1,3,5);
+$c->want_dbversion = array(1,3,6);
 $c->schema_version = 0;
 $qry = new AwlQuery( 'SELECT schema_major, schema_minor, schema_patch FROM awl_db_revision ORDER BY schema_id DESC LIMIT 1;' );
 if ( $qry->Exec('always',__LINE__,__FILE__) && $row = $qry->Fetch() ) {
@@ -601,4 +612,5 @@
   }
   return $privileges;
 }
+
 
