VALAC=valac
VALAFILES=src/xmz-mod-touch-gui.vala
VALAPKGS=--pkg gtk+-3.0
VALAOPTS=

EXEC=xmz-mod-touch-gui

default:
	$(VALAC) $(VALAFILES) -o $(EXEC) $(VALAPKGS) $(VALAOPTS)

run:
	./$(EXEC)
