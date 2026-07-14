PORTNAME=	postfix-mta-sts-resolver
PORTVERSION=	1.5.1
DISTVERSIONPREFIX=	v
CATEGORIES=	mail python
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	nork@FreeBSD.org
COMMENT=	TLS client policy for Postfix according to domain MTA-STS policy
WWW=		https://pypi.python.org/pypi/postfix-mta-sts-resolver

LICENSE=	MIT

BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}wheel>0:devel/py-wheel@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}packaging>=24.0:devel/py-packaging@${PY_FLAVOR}
RUN_DEPENDS=	${LOCALBASE}/share/certs/ca-root-nss.crt:security/ca_root_nss \
		${PYTHON_PKGNAMEPREFIX}aiodns>=3.0.0:dns/py-aiodns@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}aiohttp>=3.4.4:www/py-aiohttp@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}pyyaml>=3.12:devel/py-pyyaml@${PY_FLAVOR}

#PYSETUPTOOLS_SETUPARGS= --no-user-cfg

USES=		python
USE_GITHUB=	yes
GH_ACCOUNT=	freultwah
GH_TAGNAME=	cfba288
USE_PYTHON=	autoplist distutils
USE_RC_SUBR=	mta_sts

#USE_PYTHON=	autoplist pep517

NO_ARCH=	yes

SUB_LIST=	PYTHON_CMD=${PYTHON_CMD}
PLIST_FILES=	"@sample etc/mta-sts-daemon.yml.sample"

OPTIONS_DEFINE=		PGSQL REDIS SQLITE UVLOOP
OPTIONS_DEFAULT=	PGSQL REDIS SQLITE UVLOOP

UVLOOP_DESC=		Event loop support
PGSQL_RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}psycopg>=3.2.0:databases/py-psycopg@${PY_FLAVOR}
REDIS_RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}redis>=4.2.0:databases/py-redis@${PY_FLAVOR}
SQLITE_RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}aiosqlite>=0.10.0:databases/py-aiosqlite@${PY_FLAVOR}
UVLOOP_RUN_DEPENDS=	${PYTHON_PKGNAMEPREFIX}uvloop>=0.11.0:devel/py-uvloop@${PY_FLAVOR}

post-extract:
	${REINPLACE_CMD} -e 's|/etc/|${PREFIX}/etc/|' \
		${WRKSRC}/postfix_mta_sts_resolver/defaults.py

post-install:
	${INSTALL_DATA} ${WRKSRC}/config_examples/mta-sts-daemon.yml.internal \
			${STAGEDIR}${PREFIX}/etc/mta-sts-daemon.yml.sample

.include <bsd.port.mk>
