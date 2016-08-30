pub use self::application::App;

// Die einzelnen Unterfenster sind in seperaten Modulen organisiert
/// Anwendungsweite Konfiguration und Initalsierung
pub mod application;
/// Hauptfenster der Module
pub mod module_index;
/// Hauptfenster der Systeminformationen
pub mod sysinfo_index;
/// Hauptfenster der Systemeinstellungen
pub mod settings_index;
