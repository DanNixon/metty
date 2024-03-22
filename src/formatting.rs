use ascii_table::{Align, AsciiTable};
use cadmium_yellow::{LineName, TrainArrival, TrainEvent, TrainEventKind};
use colored::Colorize;

pub(crate) fn metro_logo() -> String {
    format!("{}", " M ".black().on_yellow())
}

pub(crate) fn table_basic() -> AsciiTable {
    let mut table = AsciiTable::default();

    table.column(0).set_header("Due").set_align(Align::Right);
    table.column(1).set_header("Line").set_align(Align::Left);
    table
        .column(2)
        .set_header("Destination")
        .set_align(Align::Left);
    table
        .column(3)
        .set_header("Last Seen")
        .set_align(Align::Left);

    table
}

pub(crate) fn format_train_due(v: &TrainArrival) -> String {
    match v {
        TrainArrival::Arrived => "Arrived".to_string(),
        TrainArrival::Due => "Due".to_string(),
        TrainArrival::DueIn(t) => format!("{}m", t.num_minutes()),
    }
}

pub(crate) fn format_line_name(v: &LineName) -> String {
    match v {
        LineName::Green => "Green".green(),
        LineName::Yellow => "Yellow".yellow(),
    }
    .to_string()
}

pub(crate) fn format_last_seen(v: &TrainEvent) -> String {
    format!(
        "{} {}",
        match v.kind {
            TrainEventKind::Approaching => "Approaching",
            TrainEventKind::Arrived => "Arrived at",
            TrainEventKind::ReadyToStart => "Departing from",
            TrainEventKind::Departed => "Departed from",
        },
        v.location
    )
}
