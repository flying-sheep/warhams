use std::ffi::OsStr;
use std::fs::{File, read_to_string};
use std::io::{BufReader, Read};
use std::path::Path;

use anyhow::{bail, Context, Result};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use zip::ZipArchive;


#[derive(Debug)]
pub struct Roster;

impl Roster {
    fn read(path: &Path) -> Result<String> {
        if path.extension() == Some(OsStr::new("rosz")) {
            let zip_file = File::open(path).context(format!("Can’t open file {:?}", path))?;
            let zip_reader = BufReader::new(zip_file);
            let mut zip = ZipArchive::new(zip_reader)?;
            let file = zip.by_index(0)?;
            let mut buf = String::with_capacity(file.size().try_into()?);
            let mut file_reader = BufReader::new(file);
            file_reader.read_to_string(&mut buf)?;
            return Ok(buf);
        } else if path.extension() == Some(OsStr::new("ros")) {
            return Ok(read_to_string(path)?)
        } else {
            bail!("Extension {:?} unknown", path.extension());
        }
    }
    
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = Self::read(path)?;
        Self::from_str(contents.as_ref())
    }
    
    pub fn from_str(contents: &str) -> Result<Self> {
        let mut reader = Reader::from_str(contents);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut skip_buf = Vec::new();
        let mut count = 0;
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
        Ok(Roster)
    }
}

#[derive(Debug, Clone)]
struct TableStat {
    #[allow(dead_code)]
    index: u8,
    rows: Vec<Vec<String>>,
}
