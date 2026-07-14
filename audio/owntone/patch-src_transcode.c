--- src/transcode.c.orig	2024-01-18 13:46:52.000000000 +0200
+++ src/transcode.c	2024-04-07 11:47:47.904629000 +0300
@@ -48,7 +48,10 @@
 #define USE_CONST_AVCODEC (LIBAVFORMAT_VERSION_MAJOR > 59) || ((LIBAVFORMAT_VERSION_MAJOR == 59) && (LIBAVFORMAT_VERSION_MINOR > 15))
 #define USE_NO_CLEAR_AVFMT_NOFILE (LIBAVFORMAT_VERSION_MAJOR > 59) || ((LIBAVFORMAT_VERSION_MAJOR == 59) && (LIBAVFORMAT_VERSION_MINOR > 15))
 #define USE_CH_LAYOUT (LIBAVCODEC_VERSION_MAJOR > 59) || ((LIBAVCODEC_VERSION_MAJOR == 59) && (LIBAVCODEC_VERSION_MINOR > 24))
+#define USE_CONST_AVIO_WRITE_PACKET (LIBAVFORMAT_VERSION_MAJOR > 61) || ((LIBAVFORMAT_VERSION_MAJOR == 61) && (LIBAVFORMAT_VERSION_MINOR > 0))
 
+#define USE_ALAC_FRAME_SIZE_HACK (LIBAVCODEC_VERSION_MAJOR > 59) || ((LIBAVCODEC_VERSION_MAJOR == 59) && (LIBAVCODEC_VERSION_MINOR > 31))
+
 // Interval between ICY metadata checks for streams, in seconds
 #define METADATA_ICY_INTERVAL 5
 // Maximum number of streams in a file that we will accept
@@ -528,6 +531,16 @@
       return -1;
     }
 
+  // airplay.c "misuses" the ffmpeg alac encoder in that it pushes frames with
+  // 352 samples even though the encoder wants 4096 (and doesn't have variable
+  // frame capability). This worked with no issues until ffmpeg 6, where it
+  // seems a frame size check was added. The below circumvents the check, but is
+  // dirty because we shouldn't be writing to this data element.
+#if USE_ALAC_FRAME_SIZE_HACK
+  if (codec_id == AV_CODEC_ID_ALAC)
+    s->codec->frame_size = 352;
+#endif
+
   encoder = avcodec_find_encoder(codec_id);
   if (!encoder)
     {
@@ -887,8 +900,14 @@
   return (ret > 0) ? ret : AVERROR_EOF;
 }
 
+#if USE_CONST_AVIO_WRITE_PACKET
 static int
+avio_evbuffer_write(void *opaque, const uint8_t *buf, int size)
+#else
+
+static int
 avio_evbuffer_write(void *opaque, uint8_t *buf, int size)
+#endif
 {
   struct avio_evbuffer *ae = (struct avio_evbuffer *)opaque;
   int ret;
