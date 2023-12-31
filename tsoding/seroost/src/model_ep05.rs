#![allow(dead_code)]
use std::{collections::HashMap, path::{PathBuf, Path}, time::SystemTime};
use serde::{Deserialize, Serialize};
use super::lexer_ep05::Lexer;

type DocFreq = HashMap<String, usize>;
type TermFreq = HashMap::<String, usize>;
type Docs = HashMap<PathBuf, Doc>;

pub trait Model {
    fn search_query(&self, query: &[char]) -> Result<Vec<(PathBuf, f32)>, ()>;
    fn requires_reindexing(&mut self, path: &PathBuf, last_modified: SystemTime) -> Result<bool, ()>;
    fn add_document(&mut self, path: PathBuf, last_modified: SystemTime, content: &[char]) -> Result<(), ()>;
}

pub struct SqliteModel {
    connection: sqlite::Connection,
}

impl SqliteModel {
    fn execute(&self, statement: &str) -> Result<(), ()> {
        self.connection.execute(statement).map_err(|err| {
            eprintln!("ERROR: could not execute query {statement}: {err}");
        })?;
        Ok(())
    }
    pub fn begin(&self) -> Result<(), ()> {
        self.execute("BEGIN;")
    }
    pub fn commit(&self) -> Result<(), ()> {
        self.execute("COMMIT;")
    }
    pub fn open(path: &Path) -> Result<Self, ()> {
        let connection = sqlite::open(path).map_err(|err| {
            eprintln!("ERROR: could not open sqlite database {path}: {err}", path = path.display());
        })?;
        let this = Self {connection};

        this.execute("
            CREATE TABLE IF NOT EXISTS Documents (
                id INTEGER NOT NULL PRIMARY KEY,
                path TEXT,
                term_count INTEGER,
                UNIQUE(path)
            );
        ")?;

        this.execute("
            CREATE TABLE IF NOT EXISTS TermFreq (
                term TEXT,
                doc_id INTEGER,
                freq INTEGER,
                UNIQUE(term, doc_id),
                FOREIGN KEY(doc_id) REFERENCES Documents(id)
            );
        ")?;

        this.execute("
        CREATE TABLE IF NOT EXISTS DocFreq (
            term TEXT,
            freq INTEGER,
            UNIQUE(term)
        );
        ")?;

        Ok(this)
    }
}

impl Model for SqliteModel {
    fn search_query(&self, _query: &[char]) -> Result<Vec<(PathBuf, f32)>, ()> {
        todo!()
    }
    fn requires_reindexing(&mut self, _path: &PathBuf, _last_modified: SystemTime) -> Result<bool, ()> {
        Ok(true)
    }
    fn add_document(&mut self, path: PathBuf, _last_modified: SystemTime, content: &[char]) -> Result<(), ()> {
        let terms = Lexer::new(content).collect::<Vec<_>>();
        let doc_id = {
            let query = "INSERT INTO Documents (path, term_count) VALUES (:path, :count)";
            let log_err = |err| {
                eprintln!("ERROR: Could not execute query {query}: {err}");
            };
            let mut stmt = self.connection.prepare(query).map_err(log_err)?;
            stmt.bind_iter::<_, (_, sqlite::Value)>([
                (":path", path.display().to_string().as_str().into()),
                (":count", (terms.len() as i64).into()),
            ]).map_err(log_err)?;
            stmt.next().map_err(log_err)?;
            unsafe {
                sqlite3_sys::sqlite3_last_insert_rowid(self.connection.as_raw())
            }
        };

        let mut tf = TermFreq::new();
        for term in Lexer::new(content) {
            if let Some(freq) = tf.get_mut(&term) {
                *freq += 1;
            } else {
                tf.insert(term, 1);
            }
        }

        for (term, freq) in &tf {
            {
                let query = "INSERT INTO Termfreq (doc_id, term, freq) VALUES (:doc_id, :term, :freq)";
                let log_err = |err| {
                    eprintln!("ERROR: Could not execute query {query}: {err}");
                };
                let mut stmt = self.connection.prepare(query).map_err(log_err)?;
                stmt.bind_iter::<_, (_, sqlite::Value)>([
                    (":doc_id", doc_id.into()),
                    (":term", term.as_str().into()),
                    (":freq", (*freq as i64).into()),
                ]).map_err(log_err)?;
                stmt.next().map_err(log_err)?;
            }

            {
                let freq = {
                    let query = "SELECT freq FROM DocFreq WHERE term =:term";
                    let log_err = |err| {
                        eprintln!("ERROR: Could not execute query {query}: {err}");
                    };
                    let mut stmt = self.connection.prepare(query).map_err(log_err)?;
                    stmt.bind_iter::<_, (_, sqlite::Value)>([
                        (":term", term.as_str().into()),
                    ]).map_err(log_err)?;    
                    match stmt.next().map_err(log_err)? {
                        sqlite::State::Row => stmt.read::<i64, _>("freq").map_err(log_err)?,
                        sqlite::State::Done => 0,
                    }              
                };

                let query = "INSERT OR REPLACE INTO DocFreq (term, freq) VALUES (:term, :freq)";
                let log_err = |err| {
                    eprintln!("ERROR: Could not execute query {query}: {err}");
                };
                let mut stmt = self.connection.prepare(query).map_err(log_err)?;
                stmt.bind_iter::<_, (_, sqlite::Value)>([
                    (":term", term.as_str().into()),
                    (":freq", (freq + 1 as i64).into()),
                ]).map_err(log_err)?;
                stmt.next().map_err(log_err)?;
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
struct Doc {
    tf: TermFreq,
    count: usize,
    last_modified: SystemTime,
}

#[derive(Default, Deserialize, Serialize)]
pub struct InMemoryModel {
    docs: Docs,
    df: DocFreq,
}

impl InMemoryModel {
    fn remove_document(&mut self, file_path: &Path) {
        if let Some(doc) = self.docs.remove(file_path) {
            for t in doc.tf.keys() {
                if let Some(f) = self.df.get_mut(t) {
                    *f -= 1;
                }
            }
        }
    }
}

impl Model for InMemoryModel {
    fn search_query(&self, query: &[char]) -> Result<Vec<(PathBuf, f32)>, ()> {
        let mut result = Vec::new();
        let tokens = Lexer::new(&query).collect::<Vec<_>>();
        for (path, doc) in &self.docs {
            let mut rank = 0.0;
            for token in &tokens {
                rank += compute_tf(&token, doc) * compute_idf(&token, self.docs.len(), &self.df);
            }
            if rank > 0.0 {
                result.push((path.clone(), rank))
            }
        }
        result.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
        result.reverse();
        Ok(result)
    }
    fn requires_reindexing(&mut self, path: &PathBuf, last_modified: SystemTime) -> Result<bool, ()> {
        if let Some(doc) = self.docs.get(path) {
            return Ok(doc.last_modified < last_modified);
        } 
        return Ok(true);
    }
    fn add_document(&mut self, file_path: PathBuf, last_modified: SystemTime, content: &[char]) -> Result<(), ()> {
        self.remove_document(&file_path);
        let mut tf = TermFreq::new();        
        let mut count = 0;
        for term in Lexer::new(&content){
            if let Some(f) = tf.get_mut(&term) {
                *f += 1;
            } else {
                tf.insert(term, 1);
            }
            count += 1;
        }

        for t in tf.keys() {
            if let Some(f) = self.df.get_mut(t) {
                *f += 1;
            } else {
                self.df.insert(t.to_string(), 1);
            }
        }      
        self.docs.insert(file_path, Doc {count, tf, last_modified});
        Ok(())
    }
}

fn compute_tf(t: &str, doc: &Doc) -> f32 {
    let n = doc.count as f32;
    let m = *doc.tf.get(t).unwrap_or(&0) as f32; 
    m / n
}

fn compute_idf(t: &str, n: usize, df: &DocFreq) -> f32 {
    let n = n as f32;
    let m = *df.get(t).unwrap_or(&1) as f32;
    return (n / m).log10();
}
