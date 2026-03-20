use crate::db::{self, Provider, ProviderHealth};
use crate::formatter;
use anyhow::Result;

pub fn execute(app: &str, ai_mode: bool) -> Result<()> {
    let conn = db::connect()?;
    let providers = Provider::get_all(&conn, app)?;

    let mut health_status: Vec<(&Provider, Option<ProviderHealth>)> = Vec::new();
    for p in &providers {
        let health = ProviderHealth::get_for_provider(&conn, app, &p.id)?;
        health_status.push((p, health));
    }

    if ai_mode {
        println!("{}", formatter::ai::format_health(app, &health_status));
    } else {
        println!("{}", formatter::human::format_health(app, &health_status));
    }

    Ok(())
}
