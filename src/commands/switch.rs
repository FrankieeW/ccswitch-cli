use crate::db::{self, Provider};
use crate::formatter;
use anyhow::Result;

pub fn execute(
    app: &str,
    provider_id: &str,
    dry_run: bool,
    confirm: Option<&str>,
    ai_mode: bool,
) -> Result<()> {
    let conn = db::connect()?;

    let from_provider = Provider::get_current(&conn, app)?;
    let to_provider = Provider::get_by_id(&conn, app, provider_id)?;

    let to_provider = match to_provider {
        Some(p) => p,
        None => {
            if ai_mode {
                println!(
                    r#"<ccswitch command="switch" error="provider_not_found"><message>Provider '{}' not found for app '{}'</message></ccswitch>"#,
                    provider_id, app
                );
            } else {
                eprintln!("Provider '{}' not found for app '{}'", provider_id, app);
            }
            anyhow::bail!("Provider not found");
        }
    };

    // Actual switch: only when --dry-run is NOT set
    if !dry_run {
        Provider::set_current(&conn, app, provider_id)?;
    }

    let from_provider_ref = from_provider.as_ref();

    if ai_mode {
        println!(
            "{}",
            formatter::ai::format_switch_result(
                app,
                from_provider_ref,
                &to_provider,
                dry_run,
                confirm
            )
        );
    } else {
        println!(
            "{}",
            formatter::human::format_switch_result(app, from_provider_ref, &to_provider, dry_run)
        );
    }

    Ok(())
}
