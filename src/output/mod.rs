pub mod csv;
pub mod json;
pub mod text;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Csv,
    Text,
}
