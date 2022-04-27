use crate::Pos;
use serde::{Deserialize, Serialize};
use hashbrown::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub enum InputType {
    Bigram,
    Trigram,
    Skipgram,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MetricAmount {
    Boolean(bool),
    Scalar(f64),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MetricTotal {
    Count(u64),
    Scalar(f64),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MetricList {
    /// bigrams contain both bigram and skipgram stats because it
    /// refers to the number of positions
    pub bigrams: HashMap<String, Metric>,
    pub trigrams: HashMap<String, Metric>,
}

pub type MetricMap = HashMap<Vec<Pos>, Vec<(String, MetricAmount)>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Metric {
    /// the name of the Ketos function that is called
    pub function: String,
    /// type of input
    pub input: InputType,
}

impl InputType {
    pub fn length(&self) -> usize {
        match self {
            InputType::Bigram | InputType::Skipgram => 2,
            InputType::Trigram => 3,
        }
    }
}

impl MetricList {
    pub fn new() -> Self {
        Self {
            bigrams: HashMap::new(),
            trigrams: HashMap::new(),
        }
    }
}

impl MetricTotal {
    pub fn add(self, amount: MetricAmount, scale: u64) -> Self {
        match self {
            Self::Count(c) => match amount {
                MetricAmount::Boolean(a) => Self::Count(c + if a { scale } else { 0 }),
                _ => self,
            },
            Self::Scalar(s) => match amount {
                MetricAmount::Scalar(a) => Self::Scalar(s + (a * scale as f64)),
                _ => self,
            },
        }
    }
}
impl MetricAmount {
    pub fn some(&self) -> bool {
        match self {
            MetricAmount::Boolean(b) => *b,
            MetricAmount::Scalar(s) => *s != 0.0,
        }
    }
}
