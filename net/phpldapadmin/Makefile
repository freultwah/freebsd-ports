PORTNAME=	phpldapadmin
DISTVERSION=	2.3.12
CATEGORIES=	net www
PKGNAMESUFFIX=	${PHP_PKGNAMESUFFIX}

MAINTAINER=	raivo@lehma.com
COMMENT=	PHP application to administer LDAP over the web
WWW=		https://github.com/leenooks/phpLDAPadmin/wiki

LICENSE=	GPLv2
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		cpe php:web,flavors tar:tgz
CPE_VENDOR=	phpldapadmin_project
USE_GITHUB=	yes
GH_ACCOUNT=	leenooks
GH_PROJECT=	phpLDAPadmin
USE_PHP=	ctype dom fileinfo filter hash iconv ldap mbstring openssl \
		pcre session tokenizer xml
IGNORE_WITH_PHP=	82 83
RUN_DEPENDS+=	php${PHP_VER}>=8.4.1:lang/php${PHP_VER}

NO_ARCH=	yes
NO_BUILD=	yes

SUB_FILES=	pkg-message
SUB_LIST=	PKGNAME=${PKGNAME} WWWOWN=${WWWOWN} WWWGRP=${WWWGRP}

OPTIONS_DEFINE=	MCRYPT
MCRYPT_DESC=	Use pecl-mcrypt to accelerate phpseclib cryptography
MCRYPT_USE=	PHP=mcrypt

# Optional mirror directory, including a trailing slash. Without a mirror,
# prepare this archive locally with make make-deps-distfile and retain it in
# DISTDIR (also shared with Poudriere). Normal builds do not run Composer/npm.
DEPS_MASTER_SITES?=
DEPS_DISTFILE=	${PORTNAME}-${DISTVERSION}-deps.tgz

.if make(make-deps-distfile)
USES+=		nodejs:lts,build
BUILD_DEPENDS+=	npm${NODEJS_SUFFIX}>0:www/npm${NODEJS_SUFFIX} \
		${LOCALBASE}/bin/composer:devel/php-composer@${PHP_FLAVOR}
.endif

.if !make(make-deps-distfile) && !defined(SKIP_DEPS_DISTFILE)
MASTER_SITES+=	${DEPS_MASTER_SITES:S/$/:deps/}
DISTFILES+=	${DEPS_DISTFILE}:deps
EXTRACT_ONLY=	${DISTFILE_DEFAULT}

pre-fetch:
	@if [ -z "${DEPS_MASTER_SITES}" ] && \
	    [ ! -f "${DISTDIR}/${DIST_SUBDIR}/${DEPS_DISTFILE}" ]; then \
		${ECHO_MSG} "Missing ${DEPS_DISTFILE} in DISTDIR."; \
		${ECHO_MSG} "Run make make-deps-distfile with network access first, or set DEPS_MASTER_SITES to an archive mirror."; \
		exit 1; \
	fi

post-extract:
	@${TAR} -xf ${DISTDIR}/${DIST_SUBDIR}/${DEPS_DISTFILE} -C ${WRKSRC}
.endif

# Keep application code root-owned. Only storage and bootstrap/cache need to
# be writable by the web server. Configuration is installed with @sample.
do-install:
	@${MKDIR} ${STAGEDIR}${WWWDIR}
.for d in app bootstrap config database public resources routes storage templates vendor
	@cd ${WRKSRC} && ${COPYTREE_SHARE} ${d} ${STAGEDIR}${WWWDIR} \
		"-not -name .gitignore -not -name '*.orig' -not -name '*.rej'"
.endfor
.for f in artisan composer.json LICENSE README.md VERSION .env.example
	${INSTALL_DATA} ${WRKSRC}/${f} ${STAGEDIR}${WWWDIR}
.endfor
	@${RM} ${STAGEDIR}${WWWDIR}/bootstrap/cache/*.php
	@${FIND} ${STAGEDIR}${WWWDIR}/config -type f -name '*.php' \
		-exec ${MV} {} {}.sample \;
	@${FIND} ${STAGEDIR}${WWWDIR}/config -type f -name '*.sample' \
		-exec ${CHMOD} 640 {} +
	@${CHMOD} 640 ${STAGEDIR}${WWWDIR}/.env.example
	@${MKDIR} ${STAGEDIR}${WWWDIR}/templates/custom
	@${FIND} ${STAGEDIR}${WWWDIR}/storage \
		${STAGEDIR}${WWWDIR}/bootstrap/cache -type d \
		-exec ${CHMOD} 750 {} +

post-install:
	@cd ${STAGEDIR}${PREFIX} && \
		${FIND} ${WWWDIR_REL} \( -type f -o -type l \) \
			! -path '${WWWDIR_REL}/.env.example' \
			! -path '${WWWDIR_REL}/config/*.sample' | \
			${SORT} >> ${TMPPLIST}
	@${ECHO_CMD} '@sample(root,${WWWGRP},0640) ${WWWDIR_REL}/.env.example ${WWWDIR_REL}/.env' >> ${TMPPLIST}
	@cd ${STAGEDIR}${PREFIX} && \
		for f in ${WWWDIR_REL}/config/*.sample; do \
			${ECHO_CMD} "@sample(root,${WWWGRP},0640) $$f $${f%.sample}"; \
		done >> ${TMPPLIST}
	@cd ${STAGEDIR}${PREFIX} && \
		${FIND} ${WWWDIR_REL} -type d | ${SORT} -r | \
		while read d; do \
			case "$$d" in \
			${WWWDIR_REL}/storage|${WWWDIR_REL}/storage/*|${WWWDIR_REL}/bootstrap/cache) \
				${ECHO_CMD} "@dir(${WWWOWN},${WWWGRP},0750) $$d" ;; \
			*) ${ECHO_CMD} "@dir $$d" ;; \
			esac; \
		done >> ${TMPPLIST}

# Maintainer-only preparation: downloads dependencies and compiles web assets.
# After refreshing the archive, run make makesum and preserve the archive in
# DISTDIR. It is deliberately not rebuilt during ordinary package builds.
make-deps-distfile: build-depends
	@cd ${.CURDIR} && ${MAKE} SKIP_DEPS_DISTFILE=yes patch
	@cd ${WRKSRC} && ${RM} -r node_modules vendor public/css/app.css public/fonts public/js/app.js public/js/manifest.js public/js/vendor.js public/mix-manifest.json public/webfonts
	@cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} composer install --no-dev --no-interaction --no-progress --prefer-dist
	@cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} npm ci --no-audit --no-fund
	@cd ${WRKSRC} && ${SETENV} ${MAKE_ENV} npm run prod
	@cd ${WRKSRC} && ${RM} -r node_modules
	@${MKDIR} ${DISTDIR}/${DIST_SUBDIR}
	@cd ${WRKSRC} && ${TAR} -czf ${DISTDIR}/${DIST_SUBDIR}/${DEPS_DISTFILE} vendor public
	@${ECHO_MSG} "Created ${DISTDIR}/${DIST_SUBDIR}/${DEPS_DISTFILE}; run make makesum."

.include <bsd.port.mk>
