--- crates/migration/src/email_v2.rs.orig	2026-01-19 11:23:35.000000000 +0000
+++ crates/migration/src/email_v2.rs	2026-02-04 08:05:07.355352000 +0000
@@ -216,8 +216,8 @@
 }
 
 #[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive, Debug)]
-pub struct LegacyMessageMetadata {
-    pub contents: Vec<LegacyMessageMetadataContents>,
+pub struct LegacyMessageMetadata<'x> {
+    pub contents: Vec<LegacyMessageMetadataContents<'x>>,
     pub blob_hash: BlobHash,
     pub size: u32,
     pub received_at: u64,
@@ -226,8 +226,8 @@
     pub raw_headers: Vec<u8>,
 }
 
-impl From<LegacyMessageMetadata> for MessageMetadata {
-    fn from(legacy: LegacyMessageMetadata) -> Self {
+impl<'x> From<LegacyMessageMetadata<'x>> for MessageMetadata {
+    fn from(legacy: LegacyMessageMetadata<'x>) -> Self {
         MessageMetadata {
             blob_body_offset: legacy
                 .contents
@@ -251,15 +251,15 @@
 }
 
 #[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive, Debug)]
-pub struct LegacyMessageMetadataContents {
+pub struct LegacyMessageMetadataContents<'x> {
     pub html_body: Vec<u16>,
     pub text_body: Vec<u16>,
     pub attachments: Vec<u16>,
-    pub parts: Vec<LegacyMessageMetadataPart>,
+    pub parts: Vec<LegacyMessageMetadataPart<'x>>,
 }
 
-impl From<LegacyMessageMetadataContents> for MessageMetadataContents {
-    fn from(contents: LegacyMessageMetadataContents) -> Self {
+impl<'x> From<LegacyMessageMetadataContents<'x>> for MessageMetadataContents {
+    fn from(contents: LegacyMessageMetadataContents<'x>) -> Self {
         MessageMetadataContents {
             html_body: contents.html_body.into_boxed_slice(),
             text_body: contents.text_body.into_boxed_slice(),
@@ -270,8 +270,8 @@
 }
 
 #[derive(rkyv::Serialize, rkyv::Deserialize, rkyv::Archive, Debug)]
-pub struct LegacyMessageMetadataPart {
-    pub headers: Vec<Header<'static>>,
+pub struct LegacyMessageMetadataPart<'x> {
+    pub headers: Vec<Header<'x>>,
     pub is_encoding_problem: bool,
     pub body: LegacyMetadataPartType,
     pub encoding: Encoding,
@@ -281,8 +281,8 @@
     pub offset_end: u32,
 }
 
-impl From<LegacyMessageMetadataPart> for MessageMetadataPart {
-    fn from(part: LegacyMessageMetadataPart) -> Self {
+impl <'x>From<LegacyMessageMetadataPart<'x>> for MessageMetadataPart {
+    fn from(part: LegacyMessageMetadataPart<'x>) -> Self {
         let flags = match part.encoding {
             Encoding::None => 0,
             Encoding::QuotedPrintable => PART_ENCODING_QP,
