use crate::db::{self, Provider};
use crate::formatter;
use anyhow::Result;

pub fn execute(app: &str, ai_mode: bool) -> Result<()> {
    let conn = db::connect()?;
    let providers = Provider::get_all(&conn, app)?;

    if providers.is_empty() {
        if ai_mode {
            println!(
                r#"<ccswitch command="list" app_type="{}"><providers/></ccswitch>"#,
                app
            );
        } else {
            println!("No providers found for app type: {}", app);
        }
        return Ok(());
    }

    if ai_mode {
        println!("{}", formatter::ai::format_providers_list(app, &providers));
    } else {
        println!(
            "{}",
            formatter::human::format_providers_table(app, &providers)
        );
    }

    Ok(())
}
