# Created by: Elvis Chiang <elvis@sslab.cs.ccu.edu.tw>

PORTNAME=	wordpress
DISTVERSION=	7.0.1
PORTEPOCH=	1
CATEGORIES=	www
MASTER_SITES=	http://wordpress.org/

MAINTAINER=	joneum@FreeBSD.org
COMMENT=	State-of-the-art semantic personal publishing platform

LICENSE=	GPLv2

CONFLICTS_INSTALL=	de-wordpress-[0-9].* ja-wordpress-[0-9].* ru-wordpress-[0-9].*
RUN_DEPENDS=	${PECL_PKGNAMEPREFIX}imagick>=2.2.2:graphics/pecl-imagick@${PHP_FLAVOR}
RUN_DEPENDS+=	${LOCALBASE}/lib/php/${PHP_EXT_DIR}/redis.so:databases/pecl-redis@${PHP_FLAVOR}

NO_ARCH=	yes
NO_BUILD=	yes
USES=		php:web cpe
USE_PHP=	ctype curl dom exif fileinfo filter ftp gd hash iconv intl mbstring mysqli opcache openssl pcre session tokenizer xml zip zlib
WRKSRC=		${WRKDIR}/${PORTNAME}


.if ${DISTVERSION:N*.*.*}
CPE_VERSION=	${DISTVERSION}.0
.endif

SUB_FILES=	pkg-message
PLIST=		${WRKDIR}/pkg-plist
PORTDOCS=	license.txt readme.html

OPTIONS_DEFINE=	DOCS

.include <bsd.port.options.mk>

pre-everything::
	@${ECHO_CMD} "If you want to upgrade, you must read upgrade document."
	@${ECHO_CMD} ""
.  if !(defined(PACKAGE_BUILDING) || defined(BATCH))
	@sleep 1
.  endif

pre-install:
	${ECHO_CMD} '@owner www' > ${PLIST}
	${ECHO_CMD} '@group www' >> ${PLIST}
	cd ${WRKSRC}/ && ${FIND} -s * -type f | ${SED} -e 's|^|%%WWWDIR%%/|' >> ${PLIST}
	${ECHO_CMD} '@owner' >> ${PLIST}
	${ECHO_CMD} '@group' >> ${PLIST}
	if [ ! -f ${WRKSRC}/wp-config-sample.php ]; then ${CP} \
		${WRKSRC}/wp-config.php.sample ${WRKSRC}/wp-config.php; fi

do-install:
	${INSTALL} -d -m 755 ${STAGEDIR}${WWWDIR}/
	${CP} -R ${WRKSRC}/ ${STAGEDIR}${WWWDIR}/
	${FIND} ${STAGEDIR}${WWWDIR}/ -type d -exec ${CHMOD} g+w {} \;

do-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}/
	cd ${WRKSRC}/ && ${INSTALL_DATA} ${PORTDOCS} ${STAGEDIR}${DOCSDIR}/

.include <bsd.port.mk>
