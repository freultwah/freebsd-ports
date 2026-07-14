# Created by: Gea-Suan Lin <gslin@gslin.org>

PORTNAME=	apache-tika
PORTVERSION=	3.3.1
PORTREVISION=	0
PORTEPOCH=	1
CATEGORIES=	textproc java
MASTER_SITES=	https://archive.apache.org/dist/tika/${PORTVERSION}/
DISTNAME=	tika-server-standard-${PORTVERSION}-bin

MAINTAINER=	mfechner@FreeBSD.org
COMMENT=	Content analysis toolkit

LICENSE=	APACHE20

RUN_DEPENDS=	bash:shells/bash

BUILD_DEPENDS+=	openjdk25-headless>0:java/openjdk25@headless
RUN_DEPENDS+=	openjdk25-headless>0:java/openjdk25@headless

JAVA_PORT=	java/openjdk25
JAVA_HOME=	${LOCALBASE}/openjdk25
JAVA=		${JAVA_HOME}/bin/java
JAVAC=		${JAVA_HOME}/bin/javac
JAR=		${JAVA_HOME}/bin/jar

SUB_LIST+=	TIKA_JAVA_HOME=${JAVA_HOME}

MAKE_ENV+=	JAVA_HOME=${JAVA_HOME}
MAKE_ENV+=	JAVA=${JAVA}
MAKE_ENV+=	JAVAC=${JAVAC}
MAKE_ENV+=	JAR=${JAR}

#USES=		cpe java tar:tgz
USES=		cpe tar:tgz
CPE_VENDOR=	apache
CPE_PRODUCT=	tika
JAVA_VERSION=	21+
JAVA_RUN=	yes
NO_BUILD=	yes
NO_ARCH=	yes
PLIST_SUB+=	PORTVERSION="${PORTVERSION}"
SUB_FILES=	pkg-message
USE_RC_SUBR=	tika
USERS=		tika
GROUPS=		${USERS}

#UID_FILES?=	${PORTSDIR}/UIDs ${OVERLAYS}/UIDs
#GID_FILES?=	${PORTSDIR}/GIDs ${OVERLAYS}/GIDs

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

CPE_PRODUCT=	tika
CPE_VENDOR=	apache

do-install:
	${FIND} ${WRKSRC} -name '*.orig' -delete
	cd ${WRKSRC} && ${COPYTREE_SHARE} . ${STAGEDIR}${PREFIX}/${CPE_PRODUCT}
	${INSTALL_SCRIPT} ${WRKSRC}/bin/tika ${STAGEDIR}${PREFIX}/${CPE_PRODUCT}/bin
	${INSTALL_DATA} ${WRKSRC}/bin/tika.in.sh ${STAGEDIR}${PREFIX}/etc/tika.in.sh.sample
	${ECHO} 'TIKA_LOGS_DIR="/var/log/tika"' >> ${STAGEDIR}${PREFIX}/etc/tika.in.sh.sample
	${ECHO} 'TIKA_PORT="9998"' >> ${STAGEDIR}${PREFIX}/etc/tika.in.sh.sample
	${ECHO} 'TIKA_PID_DIR="/var/run/tika"' >> ${STAGEDIR}${PREFIX}/etc/tika.in.sh.sample
	${ECHO} 'TIKA_HOST="127.0.0.1"' >> ${STAGEDIR}${PREFIX}/etc/tika.in.sh.sample

	${MKDIR} ${STAGEDIR}/var/db/tika
	${MKDIR} ${STAGEDIR}/var/log/tika

post-install:
	${FIND} -s ${STAGEDIR}${PREFIX}/${CPE_PRODUCT} -not -type d | ${SORT} | \
		${SED} -e 's#^${STAGEDIR}${PREFIX}/##' >> ${TMPPLIST}
	@${ECHO_CMD} "@dir(tika,tika) /var/db/tika" >> ${TMPPLIST}
	@${ECHO_CMD} "@dir(tika,tika) /var/log/tika" >> ${TMPPLIST}
	@${ECHO_CMD} "@sample etc/tika.in.sh.sample" >> ${TMPPLIST}

.include <bsd.port.mk>
