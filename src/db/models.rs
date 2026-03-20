use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub app_type: String,
    pub name: String,
    pub settings_config: serde_json::Value,
    pub website_url: Option<String>,
    pub category: Option<String>,
    pub is_current: bool,
    pub in_failover_queue: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    pub provider_id: String,
    pub app_type: String,
    pub is_healthy: bool,
    pub consecutive_failures: i32,
    pub last_success_at: Option<String>,
    pub last_failure_at: Option<String>,
    pub last_error: Option<String>,
}

/// Shared row mapper — returns (Provider, settings_config_str) so callers can
/// decide how to handle parse errors rather than silently defaulting to Null.
fn map_provider_row(row: &rusqlite::Row) -> rusqlite::Result<(Provider, String)> {
    let settings_config_str: String = row.get(3)?;
    let is_current: bool = row.get::<_, i32>(6)? == 1;
    let in_failover_queue: bool = row.get::<_, i32>(7)? == 1;
    Ok((
        Provider {
            id: row.get(0)?,
            app_type: row.get(1)?,
            name: row.get(2)?,
            settings_config: serde_json::Value::Null, // filled in below
            website_url: row.get(4)?,
            category: row.get(5)?,
            is_current,
            in_failover_queue,
        },
        settings_config_str,
    ))
}

impl Provider {
    pub fn get_all(conn: &Connection, app_type: &str) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, app_type, name, settings_config, website_url, category, is_current, in_failover_queue
             FROM providers WHERE app_type = ? ORDER BY sort_index",
        )?;

        let providers = stmt
            .query_map([app_type], |row| {
                let (mut p, cfg_str) = map_provider_row(row)?;
                p.settings_config =
                    serde_json::from_str(&cfg_str).context("Invalid settings_config JSON").unwrap_or_else(|e| {
                        serde_json::json!({ "parse_error": e.to_string(), "raw": cfg_str })
                    });
                Ok(p)
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(providers)
    }

    pub fn get_current(conn: &Connection, app_type: &str) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, app_type, name, settings_config, website_url, category, is_current, in_failover_queue
             FROM providers WHERE app_type = ? AND is_current = 1",
        )?;

        let mut rows = stmt.query([app_type])?;
        if let Some(row) = rows.next()? {
            let (mut p, cfg_str) = map_provider_row(row)?;
            p.settings_config =
                serde_json::from_str(&cfg_str).context("Invalid settings_config JSON").unwrap_or_else(|e| {
                    serde_json::json!({ "parse_error": e.to_string(), "raw": cfg_str })
                });
            Ok(Some(p))
        } else {
            Ok(None)
        }
    }

    pub fn get_by_id(conn: &Connection, app_type: &str, id: &str) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, app_type, name, settings_config, website_url, category, is_current, in_failover_queue
             FROM providers WHERE app_type = ? AND id = ?",
        )?;

        let mut rows = stmt.query(params![app_type, id])?;
        if let Some(row) = rows.next()? {
            let (mut p, cfg_str) = map_provider_row(row)?;
            p.settings_config =
                serde_json::from_str(&cfg_str).context("Invalid settings_config JSON").unwrap_or_else(|e| {
                    serde_json::json!({ "parse_error": e.to_string(), "raw": cfg_str })
                });
            Ok(Some(p))
        } else {
            Ok(None)
        }
    }

    /// Atomically set `is_current = 1` for the given provider and
    /// `is_current = 0` for all other providers of the same app type.
    pub fn set_current(conn: &Connection, app_type: &str, provider_id: &str) -> Result<()> {
        let tx = conn
            .unchecked_transaction()
            .context("Failed to open database transaction")?;

        tx.execute(
            "UPDATE providers SET is_current = 0 WHERE app_type = ?",
            [app_type],
        )
        .context("Failed to reset current providers")?;

        let rows_affected = tx
            .execute(
                "UPDATE providers SET is_current = 1 WHERE app_type = ? AND id = ?",
                params![app_type, provider_id],
            )
            .context("Failed to set current provider")?;

        if rows_affected == 0 {
            anyhow::bail!(
                "Provider '{}' not found for app type '{}'",
                provider_id,
                app_type
            );
        }

        tx.commit().context("Failed to commit switch transaction")?;
        Ok(())
    }
}

impl ProviderHealth {
    /// Returns health rows for every provider of the given app_type in a single query.
    pub fn get_all_for_app(conn: &Connection, app_type: &str) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare(
            "SELECT h.provider_id, h.app_type, h.is_healthy, h.consecutive_failures,
                    h.last_success_at, h.last_failure_at, h.last_error
             FROM provider_health h
             INNER JOIN providers p ON p.id = h.provider_id AND p.app_type = h.app_type
             WHERE h.app_type = ?",
        )?;

        let rows = stmt
            .query_map([app_type], |row| {
                Ok(ProviderHealth {
                    provider_id: row.get(0)?,
                    app_type: row.get(1)?,
                    is_healthy: row.get::<_, i32>(2)? == 1,
                    consecutive_failures: row.get(3)?,
                    last_success_at: row.get(4)?,
                    last_failure_at: row.get(5)?,
                    last_error: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }
}
