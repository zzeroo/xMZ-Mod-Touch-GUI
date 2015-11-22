namespace XMZ {

private const string version = Config.VERSION;

public class Main {
  public static int main (string[] args) {

    Gtk.disable_setlocale ();

    Intl.setlocale (LocaleCategory.ALL, "");
    Intl.setlocale (LocaleCategory.COLLATE, "C");

    Intl.bindtextdomain (Config.GETTEXT_PACKAGE, Config.XMZ_LOCALEDIR);
    Intl.bind_textdomain_codeset (Config.GETTEXT_PACKAGE, "UTF-8");
    Intl.textdomain (Config.GETTEXT_PACKAGE);

    Environment.set_prgname ("xmz");
    Environment.set_application_name (_("xmz"));

    Application app = new Application ();
    return app.run (args);
  }
}
}
