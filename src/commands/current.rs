use crate::db::{self, Provider};
use crate::formatter;
use anyhow::Result;

pub fn execute(app: &str, ai_mode: bool) -> Result<()> {
    let conn = db::connect()?;
    let provider = Provider::get_current(&conn, app)?;

    match provider {
        Some(p) => {
            if ai_mode {
                println!("{}", formatter::ai::format_current_provider(app, &p));
            } else {
                println!("{}", formatter::human::format_current_provider(app, &p));
            }
        }
        None => {
            if ai_mode {
                println!(
                    r#"<ccswitch command="current" app_type="{}"><current provider="none"/></ccswitch>"#,
                    app
                );
            } else {
                println!("No current provider set for app type: {}", app);
            }
        }
    }

    Ok(())
}
