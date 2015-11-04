namespace XMZ {

private const string version = Config.VERSION;

public class Main {
  public static int main (string[] args) {

    Environment.set_prgname ("xmz");
    Environment.set_application_name (_("xmz"));

    Application app = new Application ();
    return app.run (args);
  }
}
}
