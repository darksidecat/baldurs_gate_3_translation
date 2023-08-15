use std::fs::File;
use std::io::BufReader;
use uuid::{Uuid, uuid};
use xml::reader::EventReader;
use xml::reader::XmlEvent;
use crate::domain::LocalizationLine;

pub fn parse_translation(file: File) -> std::io::Result<Vec<LocalizationLine>> {
    let file = BufReader::new(file);
    let mut parser = EventReader::new(file);

    let mut lines = Vec::new();
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {name, attributes, ..}) =>
                {
                    if name.local_name == "content" {
                    let contentuid = attributes.get(0).unwrap().value.to_owned();
                    let version = attributes.get(1).unwrap().value.to_owned();
                        match parser.next().unwrap() {
                            XmlEvent::Characters(_text) => {
                                let line = LocalizationLine {
                                    contentuid: contentuid.clone(),
                                    version: version.clone().parse().unwrap(),
                                    text: _text.to_owned()
                                };
                                lines.push(line);
                            }
                            _ => {}
                        }
                }
                }

            Ok(XmlEvent::Characters(_)) => {}
            Ok(XmlEvent::EndDocument) => {break;}
            _ => {}
        }
    }

    Ok(lines)
}
