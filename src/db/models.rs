use anyhow::Result;
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

impl Provider {
    pub fn get_all(conn: &Connection, app_type: &str) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, app_type, name, settings_config, website_url, category, is_current, in_failover_queue
             FROM providers WHERE app_type = ? ORDER BY sort_index"
        )?;

        let providers = stmt
            .query_map([app_type], |row| {
                let settings_config_str: String = row.get(3)?;
                let settings_config: serde_json::Value =
                    serde_json::from_str(&settings_config_str).unwrap_or(serde_json::Value::Null);
                Ok(Provider {
                    id: row.get(0)?,
                    app_type: row.get(1)?,
                    name: row.get(2)?,
                    settings_config,
                    website_url: row.get(4)?,
                    category: row.get(5)?,
                    is_current: row.get::<_, i32>(6)? == 1,
                    in_failover_queue: row.get::<_, i32>(7)? == 1,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(providers)
    }

    pub fn get_current(conn: &Connection, app_type: &str) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, app_type, name, settings_config, website_url, category, is_current, in_failover_queue
             FROM providers WHERE app_type = ? AND is_current = 1"
        )?;

        let mut rows = stmt.query([app_type])?;
        if let Some(row) = rows.next()? {
            let settings_config_str: String = row.get(3)?;
            let settings_config: serde_json::Value =
                serde_json::from_str(&settings_config_str).unwrap_or(serde_json::Value::Null);
            Ok(Some(Provider {
                id: row.get(0)?,
                app_type: row.get(1)?,
                name: row.get(2)?,
                settings_config,
                website_url: row.get(4)?,
                category: row.get(5)?,
                is_current: true,
                in_failover_queue: row.get::<_, i32>(7)? == 1,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_by_id(conn: &Connection, app_type: &str, id: &str) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, app_type, name, settings_config, website_url, category, is_current, in_failover_queue
             FROM providers WHERE app_type = ? AND id = ?"
        )?;

        let mut rows = stmt.query(params![app_type, id])?;
        if let Some(row) = rows.next()? {
            let settings_config_str: String = row.get(3)?;
            let settings_config: serde_json::Value =
                serde_json::from_str(&settings_config_str).unwrap_or(serde_json::Value::Null);
            Ok(Some(Provider {
                id: row.get(0)?,
                app_type: row.get(1)?,
                name: row.get(2)?,
                settings_config,
                website_url: row.get(4)?,
                category: row.get(5)?,
                is_current: row.get::<_, i32>(6)? == 1,
                in_failover_queue: row.get::<_, i32>(7)? == 1,
            }))
        } else {
            Ok(None)
        }
    }
}

impl ProviderHealth {
    pub fn get_for_provider(
        conn: &Connection,
        app_type: &str,
        provider_id: &str,
    ) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            "SELECT provider_id, app_type, is_healthy, consecutive_failures, last_success_at, last_failure_at, last_error
             FROM provider_health WHERE app_type = ? AND provider_id = ?"
        )?;

        let mut rows = stmt.query(params![app_type, provider_id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(ProviderHealth {
                provider_id: row.get(0)?,
                app_type: row.get(1)?,
                is_healthy: row.get::<_, i32>(2)? == 1,
                consecutive_failures: row.get(3)?,
                last_success_at: row.get(4)?,
                last_failure_at: row.get(5)?,
                last_error: row.get(6)?,
            }))
        } else {
            Ok(None)
        }
    }
}
