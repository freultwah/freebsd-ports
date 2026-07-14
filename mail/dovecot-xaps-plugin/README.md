# iOS push e-mail plugin for Dovecot, FreeBSD port

The point of this exercise in futility was to create a working port of the [dovecot-xaps-plugin](https://github.com/freswa/dovecot-xaps-plugin) for the FreeBSD ports tree, meant to be used as mail/dovecot-xaps-plugin. In order to make it work, you'll have to include the 95-xaps.conf file somewhere in Dovecot's config files, like this:

`!include 95-xaps.conf`
