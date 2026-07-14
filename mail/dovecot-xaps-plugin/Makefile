PORTNAME=	dovecot-xaps-plugin
DISTVERSION=	g20210623
PORTREVISION=	0
CATEGORIES=	mail

MAINTAINER=	raivo@lehma.com
COMMENT=	Apple push notification plugin for Dovecot

LICENCE=	MIT

USE_GITHUB=	yes
GH_ACCOUNT=	freswa
GH_TAGNAME=	197d68e

USES+=		cmake

BUILD_DEPENDS+=	dovecot>=2.3.9:mail/dovecot
RUN_DEPENDS+=	dovecot>=2.3.9:mail/dovecot
RUN_DEPENDS+=	dovecot-xaps-daemon>=g20200723:mail/dovecot-xaps-daemon

LDFLAGS+=	-L${LOCALBASE}/lib
USE_LDCONFIG=	${PREFIX}/lib/dovecot

post-patch:
	@${REINPLACE_CMD} -e 's|var\/run\/dovecot|var\/run\/xapsd|' ${WRKSRC}/xaps-utils.h
	@${REINPLACE_CMD} -e 's|var\/run\/dovecot|var\/run\/xapsd|' ${WRKSRC}/xaps.conf

post-stage:
	@${MKDIR} ${STAGEDIR}${PREFIX}/etc/dovecot/
	${INSTALL_DATA} ${WRKSRC}/xaps.conf \
		${STAGEDIR}${PREFIX}/etc/dovecot/95-xaps.conf.sample
	
.include <bsd.port.mk>
