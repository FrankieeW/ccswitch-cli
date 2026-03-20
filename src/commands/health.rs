use crate::db::{self, Provider, ProviderHealth};
use crate::formatter;
use anyhow::Result;

pub fn execute(app: &str, ai_mode: bool) -> Result<()> {
    let conn = db::connect()?;
    let providers = Provider::get_all(&conn, app)?;

    // Single query: get all health rows for this app type at once.
    let health_map: std::collections::HashMap<String, ProviderHealth> =
        ProviderHealth::get_all_for_app(&conn, app)?
            .into_iter()
            .map(|h| (h.provider_id.clone(), h))
            .collect();

    let health_status: Vec<(&Provider, Option<ProviderHealth>)> = providers
        .iter()
        .map(|p| (p, health_map.get(&p.id).cloned()))
        .collect();

    if ai_mode {
        println!("{}", formatter::ai::format_health(app, &health_status));
    } else {
        println!("{}", formatter::human::format_health(app, &health_status));
    }

    Ok(())
}
