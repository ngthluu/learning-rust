use crate::db::{ToDoListRecord, ToDoListRecordStatus};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::str::FromStr;
use std::time::UNIX_EPOCH;

const SEPERATOR: &str = "|||";

pub const DB_FILE_PATH: &str = "./data/db";
const TMP_FILE_PATH: &str = "./data/tmp";

#[derive(Debug)]
pub struct FileSystem {
    pub path: String,
}

impl FileSystem {
    pub fn load_data(&self) -> io::Result<HashMap<String, ToDoListRecord>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)?;
        let reader = io::BufReader::new(file);

        let mut lst = HashMap::new();
        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            let fields: Vec<&str> = line.split(SEPERATOR).collect();

            if fields.len() < 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "[Line {}] Not enough fields: expected 4, got {}",
                        line_num + 1,
                        fields.len()
                    ),
                ));
            }

            let key = fields[0].to_string();
            let timestamp = fields[1]
                .parse::<u64>()
                .map(|secs| UNIX_EPOCH + std::time::Duration::from_secs(secs))
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let content = fields[2].to_string();
            let status = ToDoListRecordStatus::from_str(fields[3])
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            lst.insert(
                key,
                ToDoListRecord {
                    timestamp,
                    content,
                    status,
                },
            );
        }

        Ok(lst)
    }

    pub fn add(&self, key: String, record: &ToDoListRecord) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        writeln!(
            file,
            "{}{}{}{}{}{}{}",
            key,
            SEPERATOR,
            record
                .timestamp
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            SEPERATOR,
            record.content,
            SEPERATOR,
            record.status
        )?;

        Ok(())
    }

    pub fn edit(&self, key: String, record: &ToDoListRecord) -> io::Result<()> {
        let file_input = File::open(&self.path)?;
        let reader = BufReader::new(file_input);

        let file_output = File::create(TMP_FILE_PATH)?;
        let mut writer = BufWriter::new(file_output);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with(&key) {
                writeln!(
                    writer,
                    "{}{}{}{}{}{}{}",
                    key,
                    SEPERATOR,
                    record
                        .timestamp
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        .to_string(),
                    SEPERATOR,
                    record.content,
                    SEPERATOR,
                    record.status
                )?;
            } else {
                writeln!(writer, "{}", line)?;
            }
        }

        fs::rename(TMP_FILE_PATH, &self.path)?;

        Ok(())
    }
}
