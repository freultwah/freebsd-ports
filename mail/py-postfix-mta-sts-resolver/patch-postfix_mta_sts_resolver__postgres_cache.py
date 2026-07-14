--- postfix_mta_sts_resolver/postgres_cache.py.orig	2026-01-05 09:22:53.073554000 +0000
+++ postfix_mta_sts_resolver/postgres_cache.py	2026-01-05 09:24:27.034598000 +0000
@@ -3,95 +3,131 @@
 import json
 import logging
 
-import asyncpg
+import psycopg
+from psycopg import sql
+from psycopg_pool import AsyncConnectionPool
 
 from .defaults import POSTGRES_TIMEOUT
 from .base_cache import BaseCache, CacheEntry
 
-
 class PostgresCache(BaseCache):
     def __init__(self, *, timeout=POSTGRES_TIMEOUT, **kwargs):
         self._last_proactive_fetch_ts_id = 1
-        asyncpglogger = logging.getLogger("asyncpg")
-        if not asyncpglogger.hasHandlers():  # pragma: no cover
-            asyncpglogger.addHandler(logging.NullHandler())
+        psycopglogger = logging.getLogger("psycopg")
+        if not psycopglogger.hasHandlers():  # pragma: no cover
+            psycopglogger.addHandler(logging.NullHandler())
         self._timeout = timeout
         self._pool = None
         self.kwargs = kwargs
 
     async def setup(self):
         queries = [
-            "CREATE TABLE IF NOT EXISTS proactive_fetch_ts "
-            "(id serial primary key, last_fetch_ts integer)",
-            "CREATE TABLE IF NOT EXISTS sts_policy_cache "
-            "(id serial primary key, domain text, ts integer, pol_id text, pol_body jsonb)",
-            "CREATE UNIQUE INDEX IF NOT EXISTS sts_policy_domain ON sts_policy_cache (domain)",
-            "CREATE INDEX IF NOT EXISTS sts_policy_domain_ts ON sts_policy_cache (domain, ts)",
+            sql.SQL(
+                "CREATE TABLE IF NOT EXISTS proactive_fetch_ts "
+                "(id serial primary key, last_fetch_ts integer)"
+            ),
+            sql.SQL(
+                "CREATE TABLE IF NOT EXISTS sts_policy_cache "
+                "(id serial primary key, domain text, ts integer, "
+                "pol_id text, pol_body jsonb)"
+            ),
+            sql.SQL(
+                "CREATE UNIQUE INDEX IF NOT EXISTS sts_policy_domain "
+                "ON sts_policy_cache (domain)"
+            ),
+            sql.SQL(
+                "CREATE INDEX IF NOT EXISTS sts_policy_domain_ts "
+                "ON sts_policy_cache (domain, ts)"
+            ),
         ]
 
-        async def set_type_codec(conn):
-            await conn.set_type_codec(
-                'jsonb',
-                encoder=json.dumps,
-                decoder=json.loads,
-                schema='pg_catalog',
-            )
+        conninfo = self.kwargs.get("dsn") or self.kwargs
 
-        self._pool = await asyncpg.create_pool(init=set_type_codec, **self.kwargs)
-        async with self._pool.acquire(timeout=self._timeout) as conn:
+        # Prevent implicit open
+        self._pool = AsyncConnectionPool(conninfo=conninfo, open=False)
+        await self._pool.open()
+
+        async with self._pool.connection() as conn:
             async with conn.transaction():
                 for q in queries:
                     await conn.execute(q)
 
     async def get_proactive_fetch_ts(self):
-        async with self._pool.acquire(timeout=self._timeout) as conn, conn.transaction():
-            cur = await conn.cursor('SELECT last_fetch_ts FROM '
-                                    'proactive_fetch_ts where id = $1',
-                                    self._last_proactive_fetch_ts_id)
-            res = await cur.fetchrow()
+        async with self._pool.connection() as conn:
+            async with conn.cursor() as cur:
+                await cur.execute(
+                    sql.SQL("SELECT last_fetch_ts FROM proactive_fetch_ts WHERE id = %s"),
+                    [self._last_proactive_fetch_ts_id]
+                )
+                res = await cur.fetchone()
         return int(res[0]) if res is not None else 0
 
     async def set_proactive_fetch_ts(self, timestamp):
-        async with self._pool.acquire(timeout=self._timeout) as conn, conn.transaction():
-            await conn.execute("""
-                INSERT INTO proactive_fetch_ts (last_fetch_ts, id)
-                VALUES ($1, $2)
-                ON CONFLICT (id) DO UPDATE SET last_fetch_ts = EXCLUDED.last_fetch_ts
-                """,
-                int(timestamp), self._last_proactive_fetch_ts_id,
-            )
+        async with self._pool.connection() as conn:
+            async with conn.transaction():
+                await conn.execute(
+                    sql.SQL("""
+                        INSERT INTO proactive_fetch_ts (last_fetch_ts, id)
+                        VALUES (%s, %s)
+                        ON CONFLICT (id) DO UPDATE SET last_fetch_ts = EXCLUDED.last_fetch_ts
+                    """),
+                    [int(timestamp), self._last_proactive_fetch_ts_id]
+                )
 
     async def get(self, key):
-        async with self._pool.acquire(timeout=self._timeout) as conn, conn.transaction():
-            cur = await conn.cursor('SELECT ts, pol_id, pol_body FROM '
-                                    'sts_policy_cache WHERE domain=$1',
-                                    key)
-            res = await cur.fetchrow()
+        async with self._pool.connection() as conn:
+            async with conn.cursor() as cur:
+                await cur.execute(
+                    sql.SQL("SELECT ts, pol_id, pol_body FROM sts_policy_cache WHERE domain=%s"),
+                    [key]
+                )
+                res = await cur.fetchone()
         if res is not None:
             ts, pol_id, pol_body = res
             ts = int(ts)
-            return CacheEntry(ts, pol_id, pol_body)
+            # Handle different possible types of pol_body
+            if isinstance(pol_body, dict):
+                return CacheEntry(ts, pol_id, pol_body)
+            elif isinstance(pol_body, str):
+                return CacheEntry(ts, pol_id, json.loads(pol_body))
+            else:
+                return CacheEntry(ts, pol_id, None)
         else:
             return None
 
     async def set(self, key, value):
         ts, pol_id, pol_body = value
-        async with self._pool.acquire(timeout=self._timeout) as conn, conn.transaction():
-            await conn.execute("""
-                INSERT INTO sts_policy_cache (domain, ts, pol_id, pol_body) VALUES ($1, $2, $3, $4)
-                ON CONFLICT (domain) DO UPDATE
-                SET ts = EXCLUDED.ts, pol_id = EXCLUDED.pol_id, pol_body = EXCLUDED.pol_body
-                WHERE sts_policy_cache.ts < EXCLUDED.ts
-            """, key, int(ts), pol_id, pol_body)
+        # Convert dictionary to JSON string if needed
+        if isinstance(pol_body, dict):
+            pol_body_json = json.dumps(pol_body)
+        else:
+            pol_body_json = pol_body
 
+        async with self._pool.connection() as conn:
+            async with conn.transaction():
+                await conn.execute(
+                    sql.SQL("""
+                        INSERT INTO sts_policy_cache (domain, ts, pol_id, pol_body) VALUES (%s, %s, %s, %s)
+                        ON CONFLICT (domain) DO UPDATE
+                        SET ts = EXCLUDED.ts, pol_id = EXCLUDED.pol_id, pol_body = EXCLUDED.pol_body
+                        WHERE sts_policy_cache.ts < EXCLUDED.ts
+                    """),
+                    [key, int(ts), pol_id, pol_body_json]
+                )
+
     async def scan(self, token, amount_hint):
         if token is None:
             token = 1
 
-        async with self._pool.acquire(timeout=self._timeout) as conn, conn.transaction():
-            res = await conn.fetch('SELECT id, ts, pol_id, pol_body, domain FROM '
-                                    'sts_policy_cache WHERE id >= $1 ORDER BY id ASC LIMIT $2',
-                                    token, amount_hint)
+        async with self._pool.connection() as conn:
+            async with conn.cursor() as cur:
+                await cur.execute(
+                    sql.SQL("SELECT id, ts, pol_id, pol_body, domain FROM sts_policy_cache "
+                           "WHERE id >= %s ORDER BY id ASC LIMIT %s"),
+                    [token, amount_hint]
+                )
+                res = await cur.fetchall()
+
         if res:
             result = []
             new_token = token
@@ -100,11 +136,18 @@
                 ts = int(ts)
                 rowid = int(rowid)
                 new_token = max(new_token, rowid)
-                result.append((domain, CacheEntry(ts, pol_id, pol_body)))
+                # Handle different possible types of pol_body
+                if isinstance(pol_body, dict):
+                    result.append((domain, CacheEntry(ts, pol_id, pol_body)))
+                elif isinstance(pol_body, str):
+                    result.append((domain, CacheEntry(ts, pol_id, json.loads(pol_body))))
+                else:
+                    result.append((domain, CacheEntry(ts, pol_id, None)))
             new_token += 1
             return new_token, result
         else:
             return None, []
 
     async def teardown(self):
-        await self._pool.close()
+        if self._pool is not None:
+            await self._pool.close()
