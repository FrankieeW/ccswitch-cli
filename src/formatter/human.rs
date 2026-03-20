use crate::db::{Provider, ProviderHealth};
use tabled::{Table, Tabled};

pub fn format_providers_table(app: &str, providers: &[Provider]) -> String {
    let rows: Vec<ProviderRow> = providers
        .iter()
        .map(|p| ProviderRow {
            id: p.id.clone(),
            name: p.name.clone(),
            category: p.category.clone().unwrap_or_default(),
            current: if p.is_current {
                "●".to_string()
            } else {
                "".to_string()
            },
        })
        .collect();

    let mut output = format!("{} Providers:\n\n", capitalize(app));
    output.push_str(&Table::new(rows).to_string());
    output
}

pub fn format_switch_result(
    app: &str,
    from: Option<&Provider>,
    to: &Provider,
    dry_run: bool,
) -> String {
    let action = if dry_run { "Would switch" } else { "Switching" };

    let from_name = from.map(|p| p.name.as_str()).unwrap_or("none");
    let from_id = from.map(|p| p.id.as_str()).unwrap_or("-");

    format!(
        "{} {} from {} ({}) to {} ({})\n",
        capitalize(action),
        app,
        from_name,
        from_id,
        to.name,
        to.id
    )
}

pub fn format_current_provider(app: &str, provider: &Provider) -> String {
    let mut output = format!("Current {} provider:\n", capitalize(app));
    output.push_str(&format!("  ID:       {}\n", provider.id));
    output.push_str(&format!("  Name:     {}\n", provider.name));
    if let Some(ref cat) = provider.category {
        output.push_str(&format!("  Category: {}\n", cat));
    }
    if let Some(ref url) = provider.website_url {
        output.push_str(&format!("  URL:      {}\n", url));
    }
    output
}

pub fn format_health(app: &str, health_status: &[(&Provider, Option<ProviderHealth>)]) -> String {
    let mut output = format!("{} Provider Health:\n\n", capitalize(app));

    for (provider, health) in health_status {
        let status = match health {
            Some(h) => {
                if h.is_healthy {
                    "✓ Healthy"
                } else {
                    "✗ Unhealthy"
                }
            }
            None => "? Unknown",
        };

        let fail_info = match health {
            Some(h) if h.consecutive_failures > 0 => {
                format!(" ({} failures)", h.consecutive_failures)
            }
            _ => String::new(),
        };

        let marker = if provider.is_current { "● " } else { "  " };
        output.push_str(&format!(
            "{}{} - {} {}\n",
            marker, provider.name, status, fail_info
        ));

        if let Some(h) = health {
            if let Some(ref err) = h.last_error {
                output.push_str(&format!("    Last error: {}\n", truncate(err, 60)));
            }
        }
    }

    output
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn truncate(s: &str, len: usize) -> String {
    if s.len() <= len {
        s.to_string()
    } else {
        format!("{}...", &s[..len.saturating_sub(3)])
    }
}

#[derive(Tabled)]
struct ProviderRow {
    id: String,
    name: String,
    category: String,
    current: String,
}
