//! The types of `rowset`.

#[derive(Clone, Copy, Debug, Default)]
pub struct RowSettings {
    pub weather: Option<crate::weather_kinds::WeatherKind>,
}
