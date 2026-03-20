use crate::db::{Provider, ProviderHealth};
use std::fmt::Write;

const SKILL_URL: &str = "https://github.com/FrankieeW/agent-skills";
const SKILL_HINT: &str = "npx -g skills add https://github.com/FrankieeW/agent-skills";

pub fn format_providers_list(app: &str, providers: &[Provider]) -> String {
    let mut xml = format!(
        r#"<ccswitch command="list" app_type="{}" skill_url="{}">
  <providers>"#,
        escape_xml(app),
        SKILL_URL
    );

    for p in providers {
        let current_attr = if p.is_current {
            r#" is_current="true""#
        } else {
            ""
        };
        let category_attr = p
            .category
            .as_ref()
            .map(|c| format!(r#" category="{}""#, escape_xml(c)))
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
        r#"
  </providers>
  <skill_install_hint>{}</skill_install_hint>
</ccswitch>"#,
        SKILL_HINT
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
        escape_xml(app),
        action,
        SKILL_URL
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
        xml.push_str("\n  <confirm_needed>true</confirm_needed>");
        xml.push_str(&format!(
            "\n  <confirm_token>{}</confirm_token>",
            escape_xml(&confirm_token)
        ));
    } else {
        xml.push_str("\n  <would_change>false</would_change>");
        xml.push_str("\n  <confirm_needed>false</confirm_needed>");
        xml.push_str("\n  <applied>true</applied>");
    }

    xml.push_str(&format!(
        r#"
  <skill_install_hint>{}</skill_install_hint>
</ccswitch>"#,
        SKILL_HINT
    ));

    xml
}

pub fn format_current_provider(app: &str, provider: &Provider) -> String {
    // Build a complete, well-formed XML document so AI parsers never see
    // truncated output.
    format!(
        r#"<ccswitch command="current" app_type="{}" skill_url="{}">
  <provider id="{}" name="{}">
    <category>{}</category>
  </provider>
  <skill_install_hint>{}</skill_install_hint>
</ccswitch>"#,
        escape_xml(app),
        SKILL_URL,
        escape_xml(&provider.id),
        escape_xml(&provider.name),
        escape_xml(provider.category.as_deref().unwrap_or("-")),
        SKILL_HINT
    )
}

pub fn format_health(app: &str, health_status: &[(&Provider, Option<ProviderHealth>)]) -> String {
    let mut xml = format!(
        r#"<ccswitch command="health" app_type="{}" skill_url="{}">
  <providers>"#,
        escape_xml(app),
        SKILL_URL
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
        )
        .unwrap();
    }

    xml.push_str(&format!(
        r#"
  </providers>
  <skill_install_hint>{}</skill_install_hint>
</ccswitch>"#,
        SKILL_HINT
    ));

    xml
}

/// Escape characters that are illegal in XML attribute values and text nodes.
/// Order matters: `&` must be escaped first so subsequent escapes don't double-encode.
pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
