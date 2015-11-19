# xMZ-Mod-Touch
## xMesszentrale mit Modbus Interface und Touch Screen

# Installation
## Dependencies

    apt-get install libgirepository1.0-dev
    apt-get install gnome-common

## i18n and l10n

The localization and internationalisation are made with [gettext][1].

### Translation Howto

To start a translation we need a POTFILES.in. This file contains all
source code files which are translateable. To create/ update this POTFILES.in
change to the ./po dir and execute ./update-pofiles

    cd ./po
    ./update-pofiles > POTFILES.in

The next command will create a .pot template file called messages.pot
from the POTFILES.in

    intltool-update --pot --gettext-package=messages --verbose

#### Update

To update a alredy translated .po file run

    msgmerge --update --no-fuzzy-matching --backup=off de.po messages.pot

The .mo files are created by autotools. You don't have to do that by hand.

On [stackoverflow.com][3] is a very nice answer about this topic.




# Links
## l10n

[1]:(http://www.gnu.org/software/gettext/manual/html_node/xgettext-Invocation.html#xgettext-Invocation)
- http://www.gnu.org/software/gettext/manual/html_node/xgettext-Invocation.html#xgettext-Invocation
[2]:(https://ewgeny.wordpress.com/2012/05/10/supporting-multiple-languages-in-your-application-a-simple-gettext-step-by-step-example)
- https://ewgeny.wordpress.com/2012/05/10/supporting-multiple-languages-in-your-application-a-simple-gettext-step-by-step-example/
[3]:(http://stackoverflow.com/questions/739314/easiest-way-to-generate-localization-files?#740425)
- http://stackoverflow.com/questions/739314/easiest-way-to-generate-localization-files?#740425
