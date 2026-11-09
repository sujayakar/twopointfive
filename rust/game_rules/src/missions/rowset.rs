//! src/game/rowset.ts: a mission row's own hour and weather

pub use ::game_types::rowset::types;
pub use types::*;
use crate::game::Game;

use crate::query::Query;
use crate::scene_scenarios::ScenarioEntry;

/// the switch is up (rowset.ts `live`; `ROWSET_BREAK` is a check's own word and is never up here)
fn live(g: &Game) -> bool { !BREAK }
/// a row with a level file of its own that names an hour or a weather (rowset.ts `own`)
fn own(e: &ScenarioEntry) -> bool { e.level.as_deref().is_some_and(|l| !l.is_empty()) && (e.hour.is_some() || e.weather.is_some()) }

/// The row whose own hour or weather a query is started in, or none: the first row of the query's level file that names one (THE HOUR IS THE
/// LEVEL'S). (rowset.ts rowOfQuery, of words already read)
pub fn row_of(g: &Game, q: &Query) -> Option<ScenarioEntry> {
    if !live(g) { return None; }
    let level = crate::scene_scenario_list::spec_of(q)?.level.filter(|l| !l.is_empty())?;
    crate::scene_scenario_list::all_missions().into_iter().find(|e| own(e) && e.level.as_deref() == Some(level.as_str()))
}

/// The row's own hour and weather WRITTEN ONTO a query, over whatever it carried; answers what was put on. (rowset.ts rowSettings)
pub fn row_settings(g: &Game, q: &mut Query) -> RowSettings {
    let mut put = RowSettings::default();
    let Some(e) = row_of(g, q) else { return put; };
    if let Some(hour) = e.hour { q.set("hour", hour.as_str()); put.hour = Some(hour); }
    if let Some(weather) = e.weather {
        q.set("weather", weather.as_str());
        // (a row names one of the weathers a visitor can pick: each is a weather kind of the same name)
        use crate::{scene_scenarios::ScenarioEntryWeather as Row, weather_kinds::WeatherKind as Kind};
        put.weather = Some(match weather { Row::Clear => Kind::Clear, Row::Overcast => Kind::Overcast, Row::Rain => Kind::Rain, Row::Downpour => Kind::Downpour, Row::Fog => Kind::Fog, Row::Dust => Kind::Dust });
    }
    put
}
