#![allow(dead_code)]
use std::{collections::HashMap, path::{PathBuf, Path}, time::SystemTime};
use serde::{Deserialize, Serialize};
use super::lexer_ep06_pre::Lexer;

type DocFreq = HashMap<String, usize>;
type TermFreq = HashMap::<String, usize>;
type Docs = HashMap<PathBuf, Doc>;

#[derive(Deserialize, Serialize)]
struct Doc {
    tf: TermFreq,
    count: usize,
    last_modified: SystemTime,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Model {
    docs: Docs,
    df: DocFreq,
}

impl Model {
    fn remove_document(&mut self, file_path: &Path) {
        if let Some(doc) = self.docs.remove(file_path) {
            for t in doc.tf.keys() {
                if let Some(f) = self.df.get_mut(t) {
                    *f -= 1;
                }
            }
        }
    }
    pub fn requires_reindexing(&mut self, path: &PathBuf, last_modified: SystemTime) -> bool {
        if let Some(doc) = self.docs.get(path) {
            return doc.last_modified < last_modified;
        } 
        return true;
    }
    pub fn search_query(&self, query: &[char]) -> Vec<(PathBuf, f32)> {
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
        result
    }
    pub fn add_document(&mut self, file_path: PathBuf, last_modified: SystemTime, content: &[char]) {
        self.remove_document(&file_path);
        let mut tf = TermFreq::new();        
        let mut count = 0;
        for t in Lexer::new(&content){
            if let Some(f) = tf.get_mut(&t) {
                *f += 1;
            } else {
                tf.insert(t, 1);
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
