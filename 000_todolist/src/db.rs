use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;
use std::time::SystemTime;

use crate::file::{FileSystem, DB_FILE_PATH};

// short_hash
// Hash a string value and returns a short (8 character) string
fn short_hash(value: String) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{:x}", hasher.finish())[..8].to_string()
}

// To-do list record status
#[derive(Debug, PartialEq, Clone)]
pub enum ToDoListRecordStatus {
    InProgress,
    Done,
    Deleted,
}

impl FromStr for ToDoListRecordStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "In Progress" => Ok(ToDoListRecordStatus::InProgress),
            "Done" => Ok(ToDoListRecordStatus::Done),
            "Deleted" => Ok(ToDoListRecordStatus::Deleted),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

impl std::fmt::Display for ToDoListRecordStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToDoListRecordStatus::InProgress => write!(f, "In Progress"),
            ToDoListRecordStatus::Done => write!(f, "Done"),
            ToDoListRecordStatus::Deleted => write!(f, "Deleted"),
        }
    }
}

// To-do list record
// Include content, status and its creation time
#[derive(Debug, Clone)]
pub struct ToDoListRecord {
    pub timestamp: SystemTime,
    pub content: String,
    pub status: ToDoListRecordStatus,
}

// To-do list
#[derive(Debug)]
pub struct ToDoList {
    lst: HashMap<String, ToDoListRecord>,
    file_system: FileSystem,
}

impl ToDoList {
    pub fn load() -> Result<ToDoList, String> {
        let file_system = FileSystem {
            path: DB_FILE_PATH.into(),
        };

        file_system
            .load_data()
            .map(|lst| ToDoList { file_system, lst })
            .map_err(|e| e.to_string())
    }

    pub fn get_list(&self) -> Vec<(&String, &ToDoListRecord)> {
        self.lst
            .iter()
            .filter_map(|(key, value)| {
                if value.status == ToDoListRecordStatus::InProgress {
                    Some((key, value))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn count_by_status(&self, status: ToDoListRecordStatus) -> usize {
        self.lst
            .iter()
            .filter_map(|(key, value)| {
                if value.status == status {
                    Some((key, value))
                } else {
                    None
                }
            })
            .count()
    }

    pub fn add(&mut self, content: String) -> Result<String, String> {
        let key = short_hash(format!("{}{:?}", content, SystemTime::now()));
        let record = ToDoListRecord {
            timestamp: SystemTime::now(),
            status: ToDoListRecordStatus::InProgress,
            content: content,
        };
        self.lst.insert(key.clone(), record.clone());
        self.file_system
            .add(key.clone(), &record)
            .map_err(|e| e.to_string())?;

        Ok(key)
    }

    pub fn edit(&mut self, key: String, content: String) -> Result<(), String> {
        let val = self
            .lst
            .get_mut(&key)
            .ok_or_else(|| format!("[Edit {} failed] Key is not exist", key))?;

        val.content = content;
        self.file_system.edit(key, val).map_err(|e| e.to_string())
    }

    pub fn check(&mut self, key: String) -> Result<(), String> {
        let val = self
            .lst
            .get_mut(&key)
            .ok_or_else(|| format!("[Check {} failed] Key is not exist", key))?;

        val.status = ToDoListRecordStatus::Done;
        self.file_system.edit(key, val).map_err(|e| e.to_string())
    }

    pub fn delete(&mut self, key: String) -> Result<(), String> {
        let val = self
            .lst
            .get_mut(&key)
            .ok_or_else(|| format!("[Delete {} failed] Key is not exist", key))?;

        val.status = ToDoListRecordStatus::Deleted;
        self.file_system.edit(key, val).map_err(|e| e.to_string())
    }
}
