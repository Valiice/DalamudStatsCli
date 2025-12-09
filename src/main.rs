use chrono::{DateTime, Utc};
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Plugin {
    name: String,
    internal_name: String,
    download_count: u64,
    last_update: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/Valiice/DalamudPluginRepo/master/repo.json";
    
    let plugins: Vec<Plugin> = reqwest::blocking::get(url)?
        .json::<Vec<Plugin>>()?;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Plugin Name").add_attribute(Attribute::Bold).fg(Color::Green),
            Cell::new("Internal Name").add_attribute(Attribute::Bold),
            Cell::new("Downloads").add_attribute(Attribute::Bold).fg(Color::Cyan),
            Cell::new("Last Update").add_attribute(Attribute::Bold).fg(Color::Yellow),
        ]);

    let mut total_downloads = 0;

    for plugin in &plugins {
        total_downloads += plugin.download_count;

        let dt = DateTime::from_timestamp(plugin.last_update, 0).unwrap_or_default();
        let formatted_date = dt.format("%Y-%m-%d %H:%M:%S").to_string();

        table.add_row(vec![
            Cell::new(&plugin.name),
            Cell::new(&plugin.internal_name),
            Cell::new(&plugin.download_count),
            Cell::new(formatted_date),
        ]);
    }

    table.add_row(vec![
        Cell::new("TOTAL").add_attribute(Attribute::Bold),
        Cell::new(""),
        Cell::new(total_downloads).add_attribute(Attribute::Bold).fg(Color::Cyan),
        Cell::new(""),
    ]);

    println!("\n📊 Dalamud Plugin Repository Statistics");
    println!("{}", table);

    Ok(())
}