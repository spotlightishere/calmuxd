//! Implemented per RFC 5545.
//! Please refer to https://datatracker.ietf.org/doc/html/rfc5545 or similar resources.

use std::{
    error::Error,
    fmt::{self, Write},
    fs,
};

use crate::FeedConfig;
use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};

#[derive(Debug)]
/// Possible errors we may encouter throughout muxing.
enum ParserError {
    InvalidCalendar,
    MalformedCalendar,
    FmtError(fmt::Error),
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidCalendar => write!(f, "no valid calendar object was present"),
            ParserError::MalformedCalendar => write!(f, "a calendar component is malformed"),
            ParserError::FmtError(e) => write!(f, "underlying formatting error: {}", e),
        }
    }
}

impl From<fmt::Error> for ParserError {
    fn from(value: fmt::Error) -> Self {
        ParserError::FmtError(value)
    }
}

/// Obtains a calendar property's value.
/// It's expected that input is in the format of "PROPERTY:VALUE".
fn parse_property(property: &str) -> Result<&str, ParserError> {
    // We should have two components.
    match property.split_once(":") {
        Some((_, value)) => Ok(value),
        None => Err(ParserError::MalformedCalendar),
    }
}

/// Parses the contents of the given `.ics`, returning all objects within.
fn scrape_contents(calendar: String) -> Result<String, ParserError> {
    let calendar_lines: Vec<&str> = calendar.lines().collect();

    // We should have at least four lines:
    // 1. A beginning VCALENDAR
    // 2. PRODID
    // 3. VERSION
    // 4. An ending VCALENDAR
    let line_count = calendar_lines.len();
    if line_count < 4 {
        return Err(ParserError::InvalidCalendar);
    }

    // We expect our first line to be the start of a VCALENDAR object.
    // Similarly, we expect our last line to be the end of a VCALENDAR.
    //
    // TODO(spotlightishere): RFC 5545, Section 3.4 ("iCalendar Object") notes
    // multiple VCALENDARs may be present in a single stream, but for ease
    // of implementation, we will currently assume such does not occur.
    let first_line = calendar_lines.first();
    let last_line = calendar_lines.last();
    if first_line != Some(&"BEGIN:VCALENDAR") || last_line != Some(&"END:VCALENDAR") {
        return Err(ParserError::InvalidCalendar);
    }

    // Begin parsing events within.
    let mut current_component = "";
    let mut captured_lines = String::new();

    // Create an iterator that skips over our first and last lines.
    let calendar_iter = calendar_lines[1..line_count - 1].iter();
    for &current_line in calendar_iter {
        // First, check if this is a property we need to handle.
        // We only want to care about "BEGIN:" and "END:" properties,
        // signifying the start and finish of a component type.
        //
        // For our purposes, a new calendar component is present
        // if we reach a property whose name begins with a V.
        if current_line.starts_with("END:V") {
            let component_name = parse_property(current_line)?;
            // We've somehow ended on a component that we did not start with.
            // Refuse to further process.
            if current_component != component_name {
                println!("huh: {}", current_component);
                return Err(ParserError::MalformedCalendar);
            }

            // Reset the component's name for the next loop,
            // and tack this line to our buffer.
            current_component = "";

            writeln!(captured_lines, "{}", current_line)?;
        } else if current_line.starts_with("BEGIN:V") {
            // We're starting a component, yet one is ongoing.
            if !current_component.is_empty() {
                println!("huh: {}", current_component);
                return Err(ParserError::MalformedCalendar);
            }

            // Begin a new component.
            let component_name = parse_property(current_line)?;
            current_component = component_name;

            writeln!(captured_lines, "{}", current_line)?;
        } else if !current_component.is_empty() {
            // This line does not signify the beginning or ending of a component.
            //
            // All properties within component types SHOULD be captured,
            // however properties outside of components should NOT.
            // For example, we want "LOCATION:" within a VEVENT,
            // but not the "VERSION:" property (on the root iCalendar object) outside.
            //
            // Because we have an ongoing component specified, capture this line.
            writeln!(captured_lines, "{}", current_line)?;
        }
    }

    Ok(captured_lines)
}

fn formulate_calendar(feed: FeedConfig) -> Result<String, ParserError> {
    let calendar_contents =
        fs::read_to_string("./temporary.ics").expect("failed to read dummy data");
    // TODO(spotlightishere): Handle error correctly
    let event_contents = scrape_contents(calendar_contents)?;

    // Begin writing our response!
    let mut response = String::new();
    writeln!(response, "BEGIN:VCALENDAR")?;
    // Per RFC 5545, Section 3.6 ("Calendar Components"), every iCalendar
    // object must have a VERSION and PRODID property set.
    //
    // Section 3.7.4 ("Version") states that only "2.0" is acceptable.
    writeln!(response, "VERSION:2.0")?;
    // Section 3.7.3 ("Product identifier") defines the format used below.
    // Example: `-//calmud//CALMUXD 0.1.0//EN`
    let prodid = concat!("-//calmuxd//CALMUXD ", env!("CARGO_PKG_VERSION"), "//EN");
    writeln!(response, "PRODID:{}", prodid)?;

    // Next, if present, provide a calendar name.
    // (This header is primarily used by Microsoft Outlook.)
    if let Some(calendar_name) = feed.visual_name {
        writeln!(response, "X-WR-CALNAME:{}", calendar_name)?;
    }
    // Similarly, if present, provide an Apple-specific calendar color.
    if let Some(calendar_color) = feed.color {
        writeln!(response, "X-APPLE-CALENDAR-COLOR:{}", calendar_color)?;
    }

    // Lastly, we can write all of our event contents.
    write!(response, "{}", event_contents)?;

    // We're done!
    writeln!(response, "END:VCALENDAR")?;
    Ok(response)
}

pub async fn handle_feed(feed: FeedConfig) -> impl IntoResponse {
    // TODO(spotlightishere): Fetch configured feeds
    // TODO(spotlightishere): We should handle errors with a little bit more grace.
    match formulate_calendar(feed) {
        Ok(response) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/calendar")],
            response,
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "text/plain")],
            e.to_string(),
        ),
    }
}
