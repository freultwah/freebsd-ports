# Created by: Maurice Castro <maurice@castro.aus.net>
# $FreeBSD: head/www/davical/Makefile 543790 2020-07-30 14:17:45Z rigoletto $

PORTNAME=	davical
DISTVERSIONPREFIX=	r
DISTVERSION=	1.1.13
PORTREVISION=	0
CATEGORIES?=	www
PKGNAMESUFFIX=	${PHP_PKGNAMESUFFIX}

MAINTAINER=	rigoletto@FreeBSD.org
COMMENT=	Simple CalDAV server using a postgres backend

LICENSE=	GPLv2 LGPL21
LICENSE_COMB=	multi
LICENSE_FILE=	${WRKSRC}/COPYING

RUN_DEPENDS=	${PHP_PKGNAMEPREFIX}libawl>=0.64:devel/php-libawl@${PHP_FLAVOR} \
		p5-DBD-Pg>=0:databases/p5-DBD-Pg \
		p5-DBI>=0:databases/p5-DBI \
                p5-Switch>=0:lang/p5-Switch \
		p5-YAML>=0:textproc/p5-YAML \
		pwgen:sysutils/pwgen

USES=		pgsql php:flavors shebangfix

USE_GITLAB=	yes
GL_ACCOUNT=	davical-project
GL_COMMIT=	c3591225
# See töötas kindlasti
#GL_COMMIT=	014ee353
USE_PHP=	calendar curl gettext iconv imap pdo pdo_pgsql pgsql redis session xml

NO_ARCH=	yes
NO_BUILD=	yes

PORTDOCS=	README INSTALL davical_en_user_guide.odt \
		phpdoc.ini translation.rst
PORTEXAMPLES=	config

SHEBANG_FILES=	scripts/sync-remote-caldav.php
SUB_FILES=	pkg-message

OPTIONS_DEFINE=	DOCS EXAMPLES

post-extract:
	${FIND} ${WRKSRC} -name .gitignore -delete

do-install:
	${MKDIR} ${STAGEDIR}${WWWDIR}/htdocs \
		${STAGEDIR}${WWWDIR}/inc \
		${STAGEDIR}${WWWDIR}/config \
		${STAGEDIR}${DATADIR}
	# WWWDIR
	(cd ${WRKSRC}/htdocs && \
		${COPYTREE_SHARE} . ${STAGEDIR}${WWWDIR}/htdocs)
	(cd ${WRKSRC}/inc && ${COPYTREE_SHARE} . \
		${STAGEDIR}${WWWDIR}/inc "! -name always.php.in")
	${INSTALL_DATA} ${WRKSRC}/config/example-config.php \
		${STAGEDIR}${WWWDIR}/config/config.php.sample
	# DATADIR
	(cd ${WRKSRC} && ${COPYTREE_SHARE} scripts ${STAGEDIR}${DATADIR})
	(cd ${WRKSRC} && ${COPYTREE_SHARE} dba ${STAGEDIR}${DATADIR} \
		"! -name update-davical-database")
	${INSTALL_SCRIPT} ${WRKSRC}/dba/update-davical-database \
		${STAGEDIR}${DATADIR}/dba

do-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	(cd ${WRKSRC}/docs && \
		${COPYTREE_SHARE} . ${STAGEDIR}${DOCSDIR})
	${INSTALL_DATA} ${WRKSRC}/README ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${WRKSRC}/INSTALL ${STAGEDIR}${DOCSDIR}

do-install-EXAMPLES-on:
	${MKDIR} ${STAGEDIR}${EXAMPLESDIR}
	(cd ${WRKSRC} && \
		${COPYTREE_SHARE} config ${STAGEDIR}${EXAMPLESDIR})

.include <bsd.port.mk>
