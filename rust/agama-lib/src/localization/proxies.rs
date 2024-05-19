use zbus::proxy;

#[proxy(
    interface = "org.opensuse.Agama1.Locale",
    default_service = "org.opensuse.Agama1",
    default_path = "/org/opensuse/Agama1/Locale"
)]
trait Locale {
    /// Commit method
    fn commit(&self) -> zbus::Result<()>;

    /// ListKeymaps method
    fn list_keymaps(&self) -> zbus::Result<Vec<(String, String)>>;

    /// ListLocales method
    fn list_locales(&self) -> zbus::Result<Vec<(String, String, String)>>;

    /// ListTimezones method
    fn list_timezones(&self) -> zbus::Result<Vec<(String, Vec<String>)>>;

    /// Keymap property
    #[zbus(property)]
    fn keymap(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_keymap(&self, value: &str) -> zbus::Result<()>;

    /// Locales property
    #[zbus(property)]
    fn locales(&self) -> zbus::Result<Vec<String>>;
    #[zbus(property)]
    fn set_locales(&self, value: &[&str]) -> zbus::Result<()>;

    /// Timezone property
    #[zbus(property)]
    fn timezone(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_timezone(&self, value: &str) -> zbus::Result<()>;

    /// UILocale property
    #[zbus(property, name = "UILocale")]
    fn uilocale(&self) -> zbus::Result<String>;
    #[zbus(property, name = "UILocale")]
    fn set_uilocale(&self, value: &str) -> zbus::Result<()>;
}
