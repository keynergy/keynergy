use crate::analysis::{
    AnalysisError, CombinedPos, CombinedPosGroup, ErrorType, Metric, MetricAmount, MetricList,
    MetricMap, MetricTotal,
};
use crate::{Keyboard, Keys, Pos, TextData};
use itertools::Itertools;
use ketos::{Interpreter, Value};
use std::collections::HashMap;

/// An object that handles keyboard and layout analysis.
pub struct Analyzer {
    pub interpreter: Interpreter,
    pub metrics: MetricList,
    pub data: TextData,
    /// HashMap where the key is the name of a keyboard, and the value
    /// is its metric map.
    pub keyboard_stats: HashMap<String, MetricMap>,
}

pub fn classify_ngram(
    interpreter: &Interpreter,
    pg: CombinedPosGroup,
    metric: &Metric,
) -> Result<MetricAmount, AnalysisError> {
    if metric.input != pg.len() {
        return Err(AnalysisError {
            metric: metric.function.clone(),
            error: ErrorType::ArgumentError(metric.input),
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

impl Analyzer {
    /// Constructs a new Analyzer with the given metrics and text data.
    pub fn with(metrics: MetricList, data: TextData) -> Self {
        Self {
            interpreter: interpreter(),
            data,
            metrics,
            keyboard_stats: HashMap::new(),
        }
    }
    /// Calculates the metrics of the given keyboard. This is needed
    /// for the analyze_keys function.
    pub fn calculate_metrics(&mut self, kb: &Keyboard) {
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
        for p in positions.iter().combinations(2) {
            let mut amounts: Vec<(String, MetricAmount)> =
                Vec::with_capacity(self.metrics.bigrams.len());
            for m in &self.metrics.bigrams {
                let cpg: CombinedPosGroup = p
                    .iter()
                    .map(|x| cpositions.get(x).unwrap().clone())
                    .collect();
                let amount = classify_ngram(&self.interpreter, cpg, &m.1).unwrap();
                amounts.push((m.0.clone(), amount));
            }
            let positions_vec: Vec<Pos> = p.iter().map(|x| **x).collect();
            map.insert(positions_vec, amounts);
        }
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
            let pg: Vec<char> = k.iter().map(|p| *keys.pos_key(*p)).collect();
            if let Some(freq) = self.data.bigrams.get(&pg[..]) {
                for (name, amount) in metrics {
                    let mut total = totals.entry(name.clone()).or_insert(match amount {
                        MetricAmount::Boolean(_) => MetricTotal::Count(0),
                        MetricAmount::Scalar(_) => MetricTotal::Scalar(0.0),
                    });
                    println!("{:?} {:?} {:?}", pg, total, freq);

                    println!("amount: {:?}", amount);
                    *total = total.clone().add(amount.clone(), *freq);
                    println!("{:?}", total);
                }
            }
        }
        Some(totals)
    }
}

/// Creates the default Ketos interpreter for metric extension
pub fn interpreter() -> Interpreter {
    let interp = Interpreter::new();
    interp
        .run_code(
            r#"
(define LP 0)
(define LR 1)
(define LM 2)
(define LI 3)
(define LT 4)
(define RP 9)
(define RR 8)
(define RM 7)
(define RI 6)
(define RT 5)
"#,
            None,
        )
        .unwrap();
    interp.scope().register_struct_value::<CombinedPos>();
    interp
}
