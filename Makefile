#
# Simple makefile
#
PHONY: install uninstall 

install:
	cp saitekswitch /usr/games
	cp *.xml /usr/share/games/flightgear/Protocol
	mkdir -p ~/.fgfs/Nasal
	cp -r *.nas ~/.fgfs/Nasal
	cp 55-saitekpanels.conf /usr/share/X11/xorg.conf.d
	cp 55-saitek.rules /etc/udev/rules.d
	udevadm control --reload

uninstall:
	-pkill -9 saitekswitch
	-rm /usr/games/saitekswitch
	-rm /usr/share/games/flightgear/Protocol/saitekswitch.xml
	-rm ~/.fgfs/Nasal/saitekswitch.nas
	-rm /usr/share/X11/xorg.conf.d/55-saitekpanels.conf
	-rm /etc/udev/rules.d/55-saitek.rules
	udevadm control --reload






