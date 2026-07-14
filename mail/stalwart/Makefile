PORTNAME=	stalwart
DISTVERSIONPREFIX=	v
DISTVERSION=	0.16.4
PORTREVISION=	0
CATEGORIES=	mail

MAINTAINER=	orville@anislet.dev
COMMENT=	Stalwart Mail Server
WWW=		https://stalw.art/

LICENSE=		AGPLv3 SELv1
LICENSE_COMB=		multi
LICENSE_NAME_SELv1=	Stalwart Enterprise License 1.0
LICENSE_FILE_AGPLv3=	${WRKSRC}/LICENSES/AGPL-3.0-only.txt
LICENSE_FILE_SELv1=	${WRKSRC}/LICENSES/LicenseRef-SEL.txt
LICENSE_PERMS_SELv1=	pkg-mirror auto-accept

ONLY_FOR_ARCHS=		amd64

USES=		cpe cargo
USE_GITHUB=	yes
GH_ACCOUNT=	stalwartlabs
GH_PROJECT=	stalwart
USE_RC_SUBR=	stalwart

CARGO_FEATURES=	--no-default-features

#CARGO_INSTALL_PATH=	crates/main crates/cli
CARGO_INSTALL_PATH=	crates/main

OPTIONS_DEFINE=		ENTERPRISE ZENOH
OPTIONS_DEFAULT=	SQLITE
OPTIONS_MULTI=		BACKENDS
OPTIONS_MULTI_BACKENDS=	AZURE ELASTIC FOUNDATIONDB MYSQL POSTGRES REDIS ROCKSDB S3 SQLITE

OPTIONS_EXCLUDE_armv6=	ROCKSDB
OPTIONS_EXCLUDE_armv7=	ROCKSDB
OPTIONS_EXCLUDE_i386=	ROCKSDB
OPTIONS_EXCLUDE_powerpc=ROCKSDB

AZURE_DESC=		Enable Azure storage backend
BACKEND_DESC=		Database and storage backends
ELASTIC_DESC=		Enable ElasticSearch backend
ENTERPRISE_DESC=	Enable Enterprise features (require license)
FOUNDATIONDB_DESC=	Enable FoundationDB backend (not supported yet)
MYSQL_DESC=		Enable MySQL backend
POSTGRES_DESC=		Enable PostgreSQL backend
REDIS_DESC=		Enable Redis backend
ROCKSDB_DESC=		Enable RocksDB backend
S3_DESC=		Enable S3 storage backend
SQLITE_DESC=		Enable SQLite backend
ZENOH_DESC=		Enable P2P coordination via Zenoh Eclipse

AZURE_VARS=		CARGO_FEATURES+=azure
ELASTIC_VARS=		CARGO_FEATURES+=elastic
ENTERPRISE_VARS=	CARGO_FEATURES+=enterprise
FOUNDATIONDB_VARS=	CARGO_FEATURES+=foundationdb
MYSQL_VARS=		CARGO_FEATURES+=mysql
POSTGRES_VARS=		CARGO_FEATURES+=postgres
REDIS_VARS=		CARGO_FEATURES+=redis
ROCKSDB_VARS=		CARGO_FEATURES+=rocks
S3_VARS=		CARGO_FEATURES+=s3
SQLITE_VARS=		CARGO_FEATURES+=sqlite
ZENOH_VARS=		CARGO_FEATURES_main+=zenoh

.if exists(${OVERLAYS}/UIDs)
UID_FILES?=     ${OVERLAYS}/UIDs ${PORTSDIR}/UIDs
.else
UID_FILES?=     /root/ports/LOCAL/UIDs ${PORTSDIR}/UIDs
.endif

.if exists(${OVERLAYS}/GIDs)
GID_FILES?=     ${OVERLAYS}/GIDs ${PORTSDIR}/GIDs
.else
GID_FILES?=     /root/ports/LOCAL/GIDs ${PORTSDIR}/GIDs
.endif

.include <bsd.port.options.mk>

.if ${PORT_OPTIONS:MROCKSDB}
#ROCKSDB_USES+=		llvm:build
LIB_DEPENDS+=	librocksdb.so:databases/rocksdb
.endif

FOUNDATIONDB_LIB_DEPENDS+=	libfdb_c.so:databases/foundationdb73-client

USERS=	stalwart
GROUPS=	stalwart

post-install:
	@${MKDIR} ${STAGEDIR}${ETCDIR}
#	${INSTALL_DATA} ${WRKSRC}/resources/config/config.toml \
#		${STAGEDIR}${ETCDIR}/config.toml.sample

	@${MKDIR} ${STAGEDIR}/var/db/stalwart
	@${MKDIR} ${STAGEDIR}/var/db/stalwart/data

.include <bsd.port.mk>

