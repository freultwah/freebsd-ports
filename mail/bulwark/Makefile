PORTNAME=	bulwark
DISTVERSION=	1.21.1
CATEGORIES=	mail www
MASTER_SITES=	https://github.com/bulwarkmail/webmail/releases/download/${DISTVERSION}/:lite
PKGNAMESUFFIX=	-lite
DISTFILES=	bulwark-lite-${DISTVERSION}.zip:lite

MAINTAINER=	ports@FreeBSD.org
COMMENT=	Static JMAP webmail client for Stalwart (Lite edition)
WWW=		https://bulwarkmail.org/

LICENSE=	AGPLv3
LICENSE_FILE=	${WRKSRC_source}/LICENSE

USE_GITHUB=	nodefault
GH_TUPLE=	bulwarkmail:webmail:${DISTVERSION}:source

NO_ARCH=	yes
NO_BUILD=	yes
EXTRACT_ONLY=	${DISTFILE_source}
WRKSRC=		${WRKDIR}/lite
WWWDIR=		${PREFIX}/www/${PORTNAME}${PKGNAMESUFFIX}

SUB_FILES=	pkg-message
PLIST_SUB=	SOURCE_DISTFILE=${DISTFILE_source}

post-extract:
	@${MKDIR} ${WRKSRC}
	${TAR} -xf ${DISTDIR}/${DIST_SUBDIR}/bulwark-lite-${DISTVERSION}.zip \
		-C ${WRKSRC} --no-same-owner --no-same-permissions

do-install:
	@${MKDIR} ${STAGEDIR}${WWWDIR} ${STAGEDIR}${EXAMPLESDIR} \
		${STAGEDIR}${DOCSDIR}
	(cd ${WRKSRC} && ${COPYTREE_SHARE} . ${STAGEDIR}${WWWDIR} \
		"! -name config.json ! -name policy.json \
		! -name nginx.conf.example ! -name Caddyfile.example \
		! -name LITE-README.md")
	${INSTALL_DATA} ${WRKSRC}/config.json ${STAGEDIR}${WWWDIR}/config.json.sample
	${INSTALL_DATA} ${WRKSRC}/policy.json ${STAGEDIR}${WWWDIR}/policy.json.sample
	${SED} 's|/var/www/bulwark-lite|${WWWDIR}|g' \
		${WRKSRC}/nginx.conf.example > ${STAGEDIR}${EXAMPLESDIR}/nginx.conf
	${SED} 's|/var/www/bulwark-lite|${WWWDIR}|g' \
		${WRKSRC}/Caddyfile.example > ${STAGEDIR}${EXAMPLESDIR}/Caddyfile
	${INSTALL_DATA} ${WRKSRC}/LITE-README.md ${WRKSRC_source}/NOTICE \
		${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${DISTDIR}/${DIST_SUBDIR}/${DISTFILE_source} \
		${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
