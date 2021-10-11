use crate::astronomical::Galaxy;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
use svg::{
    node::element::{Circle, Line, SVG},
    Document,
};

pub fn map(galaxy: &Galaxy) {
    // Create Document with viewBox
    let mut document: SVG = Document::new().set(
        "viewBox",
        format!("{} {} {} {}", i8::MIN, i8::MIN, u8::MAX, u8::MAX),
    );
    // Display Constellations
    for (point, cons) in galaxy.constellations.iter() {
        document = document.add(
            Circle::new()
                .set("fill", "green")
                .set("cx", point.x)
                .set("cy", point.y)
                .set("r", cons.systems.len()),
        );
    }

    // Display Constellation paths
    // Use (i8, i8) for connections
    for (start, end) in galaxy.connections.iter() {
        document = document.add(
            Line::new()
                .set("stroke", "green")
                .set("x1", start.x)
                .set("y1", start.y)
                .set("x2", end.x)
                .set("y2", end.y),
        );
        //
    }

    // Open, or create, file to write to ">"
    let path = Path::new("output.svg");
    let mut file = if path.is_file() {
        OpenOptions::new().write(true).truncate(true).open(path)
    } else {
        File::create(path)
    }
    .unwrap();
    // Write to file
    file.write(document.to_string().as_str().as_bytes())
        .unwrap();
}
