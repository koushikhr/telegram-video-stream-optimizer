use anyhow::Result;
use tracing::info;

pub fn put_system_to_sleep() -> Result<()> {
    info!("Triggering system sleep / suspend request");

    #[cfg(windows)]
    {
        unsafe {
            // SetSuspendState(Hibernate, ForceCritical, DisableWakeEvent)
            // 0 = Sleep (not Hibernate), 0 = Normal suspension, 0 = Allow wake
            windows_sys::Win32::System::Power::SetSuspendState(0, 0, 0);
        }
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("pmset")
            .arg("sleepnow")
            .status()?;
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("systemctl")
            .arg("suspend")
            .status()?;
        return Ok(());
    }

    #[allow(unreachable_code)]
    Ok(())
}
