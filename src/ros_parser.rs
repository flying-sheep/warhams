use std::path::Path;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

#[derive(Debug, Clone)]
struct TableStat {
    #[allow(dead_code)]
    index: u8,
    rows: Vec<Vec<String>>,
}

pub(crate) fn parse_roster(path: &Path) -> quick_xml::Result<()> {
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut skip_buf = Vec::new();
    let mut count = 0;
    let mut reader = Reader::from_file("tests/documents/document.xml")?;
    let mut found_tables = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(element) => match element.name().as_ref() {
                b"w:tbl" => {
                    count += 1;
                    let mut stats = TableStat {
                        index: count,
                        rows: vec![],
                    };
                    // must define stateful variables
                    // outside the nested loop else they are overwritten
                    let mut row_index = 0;
                    loop {
                        skip_buf.clear();
                        match reader.read_event_into(&mut skip_buf)? {
                            Event::Start(element) => match element.name().as_ref() {
                                b"w:tr" => {
                                    stats.rows.push(vec![]);
                                    row_index = stats.rows.len() - 1;
                                }
                                b"w:tc" => {
                                    stats.rows[row_index].push(
                                        String::from_utf8(element.name().as_ref().to_vec())
                                            .unwrap(),
                                    );
                                }
                                _ => {}
                            },
                            Event::End(element) => {
                                if element.name().as_ref() == b"w:tbl" {
                                    found_tables.push(stats);
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    dbg!(found_tables);
    Ok(())
}
