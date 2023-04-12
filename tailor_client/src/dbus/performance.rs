use zbus::{dbus_proxy, fdo};

#[dbus_proxy(
    interface = "com.tux.Tailor.Performance",
    default_service = "com.tux.Tailor",
    default_path = "/com/tux/Tailor"
)]
trait Performance {
    /// Temporarily override the performance profile. Please note that this will not survive a
    /// restart as the performance profile is handled by the overall profile configuration.
    async fn set_performance_profile(&self, name: &str, value: &str) -> fdo::Result<()>;

    /// Read the current performance profile.
    async fn get_performance_profile(&self, name: &str) -> fdo::Result<String>;

    /// Read the list of supported performance profiles.
    async fn list_performance_profiles(&self) -> fdo::Result<Vec<String>>;
}
