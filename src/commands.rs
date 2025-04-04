use crate::{LineArgs, TimesArgs};
use ascii_table::AsciiTable;
use cadmium_yellow::{Client, LineName};

pub(crate) async fn print_stations(client: &Client) -> crate::Result<()> {
    let mut stations = client
        .stations()
        .await
        .map_err(|e| format!("Failed to list stations: {e}"))?;

    stations.sort();

    for station in stations {
        println!("{} - {}", station.code, station.name);
        for platform in station.platforms {
            println!("  Platform {} {}", platform.number, platform.help_text);
        }
    }

    Ok(())
}

pub(crate) async fn print_line(client: &Client, args: LineArgs) -> crate::Result<()> {
    let line = match args.line.to_lowercase().as_str() {
        "green" => LineName::Green,
        "yellow" => LineName::Yellow,
        _ => return Err(format!("Unknown line: {}", args.line)),
    };

    let line = client
        .lines()
        .map_err(|e| format!("Failed to get lines: {e}"))?
        .into_iter()
        .find(|l| l.name == line)
        .unwrap();

    let station_names = client
        .station_names()
        .await
        .map_err(|e| format!("Failed to get station names: {e}"))?;

    let data = vec![vec![
        crate::formatting::metro_logo(),
        format!("{} line", crate::formatting::format_line_name(&line.name)),
    ]];

    AsciiTable::default().print(data);

    let data: Vec<Vec<String>> = line
        .stations
        .into_iter()
        .map(|s| vec![s.clone(), station_names[&s].clone()])
        .collect();

    AsciiTable::default().print(data);

    Ok(())
}

pub(crate) async fn print_times(client: &Client, args: TimesArgs) -> crate::Result<()> {
    // Find station and platform
    let stations = client
        .stations()
        .await
        .map_err(|e| format!("Failed to list stations: {e}"))?;

    let station_code = args.station.to_uppercase();
    let station = stations
        .iter()
        .find(|s| s.code == station_code)
        .ok_or_else(|| format!("No station with code {station_code}"))?;

    let platform_number = args.platform;
    let platform = station
        .platforms
        .iter()
        .find(|p| p.number == platform_number)
        .ok_or_else(|| format!("No platform with number {platform_number}"))?;

    // Show station and platform details
    let data = vec![
        vec![
            crate::formatting::metro_logo(),
            format!("{} ({})", station.name, station.code),
        ],
        vec![
            String::default(),
            format!(
                "Platform {}, for trains {}",
                platform.number, platform.help_text
            ),
        ],
    ];

    AsciiTable::default().print(data);

    // Get arrivals
    let trains = client
        .trains(station, platform)
        .await
        .map_err(|e| format!("Failed to list trains: {e}"))?;

    // Show platform arrivals
    let data: Vec<Vec<String>> = trains
        .into_iter()
        .map(|train| {
            vec![
                crate::formatting::format_train_due(&train.due),
                crate::formatting::format_line_name(&train.line),
                train.destination,
                crate::formatting::format_last_seen(&train.last_event),
                train.id,
            ]
        })
        .collect();

    crate::formatting::table_basic().print(data);

    Ok(())
}
