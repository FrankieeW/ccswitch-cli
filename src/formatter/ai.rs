use crate::db::{Provider, ProviderHealth};
use std::fmt::Write;

pub fn format_providers_list(app: &str, providers: &[Provider]) -> String {
    let skill_url = "https://github.com/FrankieeW/agent-skills";
    let skill_hint = "npx -g skills add https://github.com/FrankieeW/agent-skills";

    let mut xml = format!(
        r#"<ccswitch command="list" app_type="{}" skill_url="{}">
  <providers>"#,
        app, skill_url
    );

    for p in providers {
        let current_attr = if p.is_current {
            " is_current=\"true\""
        } else {
            ""
        };
        let category_attr = p
            .category
            .as_ref()
            .map(|c| format!(" category=\"{}\"", c))
            .unwrap_or_default();
        write!(
            xml,
            "\n    <provider id=\"{}\" name=\"{}\"{}{}/>",
            escape_xml(&p.id),
            escape_xml(&p.name),
            category_attr,
            current_attr
        )
        .unwrap();
    }

    xml.push_str(&format!(
        "
  </providers>
  <skill_install_hint>{}</skill_install_hint>
</ccswitch>",
        skill_hint
    ));

    xml
}

pub fn format_switch_result(
    app: &str,
    from: Option<&Provider>,
    to: &Provider,
    dry_run: bool,
    confirm: Option<&str>,
) -> String {
    let skill_url = "https://github.com/FrankieeW/agent-skills";
    let skill_hint = "npx -g skills add https://github.com/FrankieeW/agent-skills";
    let action = if dry_run { "dry_run" } else { "execute" };

    let confirm_token = confirm.map(|c| c.to_string()).unwrap_or_else(|| {
        if dry_run {
            "required".to_string()
        } else {
            "none".to_string()
        }
    });

    let mut xml = format!(
        r#"<ccswitch command="switch" app_type="{}" action="{}" skill_url="{}">
  <from>"#,
        app, action, skill_url
    );

    if let Some(p) = from {
        write!(
            xml,
            "\n    <provider id=\"{}\" name=\"{}\"/>",
            escape_xml(&p.id),
            escape_xml(&p.name)
        )
        .unwrap();
    } else {
        xml.push_str("\n    <provider id=\"-\" name=\"none\"/>");
    }

    xml.push_str("\n  </from>");
    xml.push_str("\n  <to>");
    write!(
        xml,
        "\n    <provider id=\"{}\" name=\"{}\"/>",
        escape_xml(&to.id),
        escape_xml(&to.name)
    )
    .unwrap();
    xml.push_str("\n  </to>");

    if dry_run {
        xml.push_str("\n  <would_change>true</would_change>");
        xml.push_str(&format!("\n  <confirm_needed>true</confirm_needed>"));
        xml.push_str(&format!(
            "\n  <confirm_token>{}</confirm_token>",
            confirm_token
        ));
    } else {
        xml.push_str("\n  <would_change>false</would_change>");
        xml.push_str("\n  <confirm_needed>false</confirm_needed>");
        xml.push_str("\n  <applied>true</applied>");
    }

    xml.push_str(&format!(
        "
  <skill_install_hint>{}</skill_install_hint>
</ccswitch>",
        skill_hint
    ));

    xml
}

pub fn format_current_provider(app: &str, provider: &Provider) -> String {
    let skill_url = "https://github.com/FrankieeW/agent-skills";
    let _skill_hint = "npx -g skills add https://github.com/FrankieeW/agent-skills";

    format!(
        r#"<ccswitch command="current" app_type="{}" skill_url="{}">
  <provider id="{}" name="{}">
    <category>{}</category>"#,
        app,
        skill_url,
        escape_xml(&provider.id),
        escape_xml(&provider.name),
        escape_xml(provider.category.as_deref().unwrap_or("-"))
    )
    .into()
}

pub fn format_health(app: &str, health_status: &[(&Provider, Option<ProviderHealth>)]) -> String {
    let skill_url = "https://github.com/FrankieeW/agent-skills";
    let skill_hint = "npx -g skills add https://github.com/FrankieeW/agent-skills";

    let mut xml = format!(
        r#"<ccswitch command="health" app_type="{}" skill_url="{}">
  <providers>"#,
        app, skill_url
    );

    for (provider, health) in health_status {
        let healthy = health.as_ref().map(|h| h.is_healthy).unwrap_or(false);
        let failures = health.as_ref().map(|h| h.consecutive_failures).unwrap_or(0);

        write!(
            xml,
            "\n    <provider id=\"{}\" name=\"{}\" is_current=\"{}\" healthy=\"{}\" consecutive_failures=\"{}\"/>",
            escape_xml(&provider.id),
            escape_xml(&provider.name),
            provider.is_current,
            healthy,
            failures
        ).unwrap();
    }

    xml.push_str(&format!(
        "
  </providers>
  <skill_install_hint>{}</skill_install_hint>
</ccswitch>",
        skill_hint
    ));

    xml
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
