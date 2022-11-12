use crate::config;
use crate::Result;
// Pretty print of tables
use prettytable::{
    self, format::FormatBuilder, format::LinePosition, format::LineSeparator, Attr, Cell, Row,
};
use serde::Serialize;
use serde_json;

pub trait AsRow {
    fn titles() -> Vec<String>;
    fn columns(&self) -> Vec<String>;
}

pub fn print_table<T: Serialize + AsRow>(rows: &[T]) -> Result<()> {
    let config = config::get_config()?;
    let if_pretty = !config.json;
    match if_pretty {
        true => print_table_asrow(rows),
        false => print_table_serde(rows),
    }
}

fn print_table_serde<T: Serialize>(rows: &[T]) -> Result<()> {
    let lines = serde_json::to_string(rows)?;
    println!("{}", lines);
    Ok(())
}

fn print_table_asrow<T: AsRow>(rows: &[T]) -> Result<()> {
    let mut table = prettytable::Table::new();
    // Define the format: it's like prettytable::format::const:FORMAT_BOX_CHARS
    // but without line separator
    let format = FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separator(LinePosition::Top, LineSeparator::new('─', '┬', '┌', '┐'))
        .separator(LinePosition::Title, LineSeparator::new('─', '┼', '├', '┤'))
        .separator(LinePosition::Bottom, LineSeparator::new('─', '┴', '└', '┘'))
        .padding(1, 1)
        .build();
    table.set_format(format);
    // Title
    let titles: Vec<Cell> = T::titles()
        .iter()
        .map(|x| Cell::new(x).with_style(Attr::Bold))
        .collect();
    table.set_titles(Row::new(titles));
    // Then add each row
    for row in rows {
        let columns = row.columns();
        table.add_row(Row::from(columns));
    }
    // Finally print
    table.printstd();
    Ok(())
}
