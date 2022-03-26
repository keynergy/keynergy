use crate::analysis::{
    AnalysisError, CombinedPos, CombinedPosGroup, ErrorType, InputType, Metric, MetricAmount,
    MetricList, MetricMap, MetricTotal,
};
use crate::{Keyboard, Keys, Pos, TextData};
use itertools::Itertools;
use ketos::{Interpreter, Value};
use std::collections::HashMap;

/// An object that handles keyboard and layout analysis.
pub struct Analyzer<'a> {
    pub interpreter: Interpreter,
    pub metrics: MetricList,
    pub data: &'a TextData,
    /// HashMap where the key is the name of a keyboard, and the value
    /// is its metric map.
    pub keyboard_stats: HashMap<String, MetricMap>,
}

pub fn classify_ngram(
    interpreter: &Interpreter,
    pg: &CombinedPosGroup,
    metric: &Metric,
) -> Result<MetricAmount, AnalysisError> {
    if metric.input.length() != pg.len() {
        return Err(AnalysisError {
            metric: metric.function.clone(),
            error: ErrorType::ArgumentError(metric.input.length()),
        });
    }
    let result = interpreter.call(
        &metric.function,
        pg.iter().map(|x| x.clone().into()).collect(),
    );
    match result {
        Ok(Value::Bool(b)) => Ok(MetricAmount::Boolean(b)),
        Ok(Value::Integer(i)) => Ok(MetricAmount::Scalar(i.to_f64().unwrap())),
        Ok(Value::Float(f)) => Ok(MetricAmount::Scalar(f)),
        Err(e) => Err(AnalysisError {
            metric: metric.function.clone(),
            error: ErrorType::Ketos(e),
        }),
        Ok(x) => Err(AnalysisError {
            metric: metric.function.clone(),
            error: ErrorType::ReturnError(x),
        }),
    }
}

impl<'a> Analyzer<'a> {
    /// Constructs a new Analyzer with the given metrics and text data.
    pub fn with(metrics: MetricList, data: &'a TextData) -> Self {
        Self {
            interpreter: interpreter(),
            data,
            metrics,
            keyboard_stats: HashMap::new(),
        }
    }
    /// Calculates the metrics of the given keyboard. This is needed
    /// for the analyze_keys function.
    pub fn calculate_metrics(&mut self, kb: &Keyboard) -> Result<(), AnalysisError> {
        let map = self
            .keyboard_stats
            .entry(kb.name.clone())
            .or_insert(HashMap::new());
        let mut positions: Vec<Pos> = Vec::with_capacity(kb.dimensions[0] * kb.dimensions[1]);
        let mut cpositions: HashMap<Pos, CombinedPos> = HashMap::new();
        for x in 0..kb.dimensions[0] {
            for y in 0..kb.dimensions[1] {
                let p = Pos::new(x, y);
                positions.push(p.clone());
                cpositions.insert(p, CombinedPos::from(kb, p));
            }
        }

        for number in 2..3 {
            for p in positions.iter().combinations(number) {
                let mut amounts: Vec<(String, MetricAmount)> =
                    Vec::with_capacity(self.metrics.bigrams.len());
                let cpg: CombinedPosGroup = p
                    .iter()
                    .map(|x| cpositions.get(x).unwrap().clone())
                    .collect();
                for m in &self.metrics.bigrams {
                    let amount = classify_ngram(&self.interpreter, &cpg, &m.1)?;
                    if amount.some() {
                        amounts.push((m.0.clone(), amount));
                    }
                }
                let positions_vec: Vec<Pos> = p.iter().map(|x| **x).collect();
                if amounts.len() > 0 {
                    map.insert(positions_vec, amounts);
                }
            }
        }
        Ok(())
    }

    /// Analyzes the keys as they would be mapped onto the given
    /// keyboard and returns the metric totals. Returns None if the
    /// keyboard metrics for the given keyboard have not been
    /// calculated yet with calculate_metrics.
    pub fn analyze_keys(&self, kb: Keyboard, keys: Keys) -> Option<HashMap<String, MetricTotal>> {
        let map = match self.keyboard_stats.get(&kb.name) {
            Some(m) => m,
            None => return None,
        };

        let mut totals: HashMap<String, MetricTotal> = HashMap::new();
        for (k, metrics) in map {
            let pg: Vec<char> = k
                .iter()
                .filter_map(|p| keys.pos_key(*p))
                .map(|p| *p)
                .collect();
            for (name, amount) in metrics {
                let freq = match pg.len() {
                    2 => match self.metrics.bigrams[name].input {
                        InputType::Bigram => self.data.bigrams.get(&pg[..]),
                        InputType::Skipgram => self.data.skip_1_grams.get(&pg[..]),
                        _ => Some(&0),
                    },
                    3 => self.data.trigrams.get(&pg[..]),
                    _ => Some(&0),
                };
                let freq = match freq {
                    Some(f) => f,
                    None => continue,
                };
                let total = totals.entry(name.clone()).or_insert(match amount {
                    MetricAmount::Boolean(_) => MetricTotal::Count(0),
                    MetricAmount::Scalar(_) => MetricTotal::Scalar(0.0),
                });
                *total = total.clone().add(amount.clone(), *freq);
            }
        }
        Some(totals)
    }
    pub fn run_ket_code(&mut self, c: String) -> Result<ketos::Value, ketos::Error> {
        self.interpreter.run_code(&c, None)
    }
    pub fn trace(
        &self,
        r: Result<ketos::Value, ketos::Error>,
    ) -> Result<ketos::Value, ketos::Error> {
        match r {
            Err(ref e) => {
                self.interpreter
                    .display_trace(&ketos::trace::take_traceback().unwrap());
                self.interpreter.display_error(e);
            }
            _ => (),
        };
        r
    }
}

/// Creates the default Ketos interpreter for metric extension
pub fn interpreter() -> Interpreter {
    let interp = Interpreter::new();
    let result = interp.run_code(include_str!("data.ket"), None);
    match result {
        Err(e) => {
            interp.display_trace(&ketos::trace::take_traceback().unwrap());
            interp.display_error(&e);
        }
        _ => (),
    };
    interp.scope().register_struct_value::<CombinedPos>();
    interp
}
