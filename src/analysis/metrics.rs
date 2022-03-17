use std::collections::HashMap;
use crate::Pos;

#[derive(Debug, PartialEq, Clone)]
pub enum MetricAmount {
    Boolean(bool),
    Scalar(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricTotal {
    Count(u64),
    Scalar(f64),
}

pub struct MetricList {
    pub bigrams: HashMap<String, Metric>,
    pub trigrams: HashMap<String, Metric>,
    pub skipgrams: HashMap<String, Metric>,
}

pub type MetricMap = HashMap<Vec<Pos>, Vec<(String, MetricAmount)>>;

pub struct Metric {
    /// the name of the Ketos function that is called
    pub function: String,
    /// number of inputs - 2 for bigram, 3 for trigram
    pub input: usize,
}

impl MetricList {
    pub fn new() -> Self {
	Self {
	    bigrams: HashMap::new(),
	    trigrams: HashMap::new(),
	    skipgrams: HashMap::new(),
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
