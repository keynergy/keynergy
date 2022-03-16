use std::collections::HashMap;
use crate::{fingers::*, Keyboard, Pos};
use itertools::{Combinations, Itertools};
use ketos::{FromValueRef, Interpreter, Value};
use ketos_derive::{ForeignValue, FromValueClone, IntoValue, StructValue};

#[derive(Debug, PartialEq)]
pub enum MetricAmount {
    Boolean(bool),
    Scalar(f64),
}

pub struct MetricList {
    bigrams: HashMap<String, Metric>,
    trigrams: HashMap<String, Metric>
}

pub struct Metric {
    /// the name of the Ketos function that is called
    function: String,
    /// number of inputs - 2 for bigram, 3 for trigram
    input: usize,
}

pub type CombinedPosGroup = Vec<CombinedPos>;

#[derive(Debug)]
pub enum ErrorType {
    Ketos(ketos::Error),
    ArgumentError(usize),
    ReturnError(Value)
}

#[derive(Debug)]
pub struct AnalysisError {
    metric: String,
    error: ErrorType
}

/// Measures the given metric for an ngram
pub fn classify_ngram(
    pg: CombinedPosGroup,
    interp: &Interpreter,
    metric: &Metric,
) -> Result<MetricAmount, AnalysisError> {
    if metric.input != pg.len() {
        return Err(AnalysisError {
	    metric: metric.function.clone(),
	    error: ErrorType::ArgumentError(metric.input)});
    }
    let result = interp
        .call(
	    &metric.function,
	    pg.iter().map(|x| x.clone().into()).collect(),
        );
    match result {
	Ok(Value::Bool(b)) => Ok(MetricAmount::Boolean(b)),
	Ok(Value::Integer(i)) => Ok(MetricAmount::Scalar(i.to_f64().unwrap())),
	Ok(Value::Float(f)) => Ok(MetricAmount::Scalar(f)),
	Err(e) => Err(AnalysisError {
	    metric: metric.function.clone(),
	    error: ErrorType::Ketos(e)
	}),
	Ok(x) => Err(AnalysisError {
	    metric: metric.function.clone(),
	    error: ErrorType::ReturnError(x)
	})
    }
}

#[derive(Clone, Debug, IntoValue, FromValueClone, ForeignValue, StructValue)]
/// A structure that combines the finger that is used and the actual
/// position data, used for Ketos metric functions
pub struct CombinedPos {
    x: f64,
    y: f64,
    finger: u8,
}

impl CombinedPos {
    pub fn from(kb: &Keyboard, p: Pos) -> Self {
        Self {
	    x: kb.rowstagger[p.row] + p.col as f64,
	    y: kb.colstagger[p.col] + p.row as f64,
	    finger: match kb.fingers.matrix[p.row][p.col] {
                LP => 0,
                LR => 1,
                LM => 2,
                LI => 3,
                LT => 4,

                RP => 9,
                RR => 8,
                RM => 7,
                RI => 6,
                RT => 5,
	    },
        }
    }
    /// creates a CombinedPosGroup from a Vector of Positions
    pub fn from_group(kb: &Keyboard, p: Vec<&Pos>) -> CombinedPosGroup {
	p.iter().map(|x| CombinedPos::from(&kb, **x)).collect::<CombinedPosGroup>()
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

pub fn bake_metrics<'a>(kb: &Keyboard, interp: &Interpreter, metrics: &'a MetricList) -> HashMap<Vec<Pos>, Vec<(&'a str, MetricAmount)>> {
    let mut map: HashMap<Vec<Pos>, Vec<(&'a str, MetricAmount)>> = HashMap::new();
    let mut positions: Vec<Pos> = Vec::with_capacity(kb.dimensions[0]*kb.dimensions[1]);
    let mut cpositions: HashMap<Pos, CombinedPos> = HashMap::new();
    for x in 0..kb.dimensions[0] {
	for y in 0..kb.dimensions[1] {
	    let p = Pos::new(x, y);
	    positions.push(p.clone());
	    cpositions.insert(p, CombinedPos::from(kb, p));
	}
    }
    for p in positions.iter().combinations(2) {
	let mut amounts: Vec<(&'a str, MetricAmount)> = Vec::with_capacity(metrics.bigrams.len());
	for m in &metrics.bigrams {
	    let cpg: CombinedPosGroup = p.iter().map(|x| cpositions.get(x).unwrap().clone()).collect();
	    let amount = classify_ngram(cpg, &interp, &m.1).unwrap();
	    amounts.push((m.0, amount));
	}
	map.insert(p.iter().map(|x| **x).collect(), amounts);
    }
    map
}

#[cfg(test)]
mod tests {
    use crate::{Fingermap, analysis::classify_ngram, fingers::*};

    use super::*;
    #[test]
    fn interpret() {
        let interp = interpreter();
        let a = CombinedPos {
	    x: 0.0,
	    y: 0.0,
	    finger: 4,
        };
        let b = CombinedPos {
	    x: 1.0,
	    y: 2.0,
	    finger: 4,
        };
        let c = CombinedPos {
	    x: 0.0,
	    y: 1.0,
	    finger: 5,
        };
        interp
	    .run_code(
                r#"
(define (sfb a b)
(= (. a :finger) (. b :finger)))
"#,
                None,
	    )
	    .unwrap();
        let result = interp
	    .call("sfb", vec![a.clone().into(), b.into()])
	    .unwrap();
        let b = bool::from_value_ref(&result).unwrap();
        assert_eq!(b, true);

        let result = interp
	    .call("sfb", vec![a.clone().into(), c.into()])
	    .unwrap();
        let b = bool::from_value_ref(&result).unwrap();
        assert_eq!(b, false);
    }
    #[test]
    fn classify() {
        let interp = interpreter();
        interp
	    .run_code(
                r#"
(define (sfb? a b)
(= (. a :finger) (. b :finger)))
"#,
                None,
	    )
	    .unwrap();

	
	let matrix = Keyboard {
	    name: "Matrix".to_string(),
	    rowstagger: vec![0.0, 0.0, 0.0],
	    colstagger: vec![0.0, 0.0, 0.0],
	    dimensions: [10, 3],
	    keyheight: 0.5,
	    fingers: Fingermap {
		matrix: vec![vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
			     vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
			     vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP]],
		map: HashMap::new(),
            },
	};
	let sfb = Metric {
	    function: "sfb?".to_string(),
	    input: 2
	};
	
	let a = Pos::new(0, 0);
        let b = Pos::new(0, 1);
	let cpg = CombinedPos::from_group(&matrix, vec![&a, &b]);
	let result = classify_ngram(cpg, &interp, &sfb).unwrap();
	assert_eq!(result, MetricAmount::Boolean(true));

	let a = Pos::new(2, 0);
        let b = Pos::new(1, 1);
	let cpg = CombinedPos::from_group(&matrix, vec![&a, &b]);
	let result = classify_ngram(cpg, &interp, &sfb).unwrap();
	assert_eq!(result, MetricAmount::Boolean(false));
    }
    #[test]
    fn bake() {
	let interp = interpreter();
        interp
	    .run_code(
                r#"
(define (sfb? a b)
(= (. a :finger) (. b :finger)))
"#,
                None,
	    )
	    .unwrap();

	let matrix = Keyboard {
	    name: "Matrix".to_string(),
	    rowstagger: vec![0.0, 0.0, 0.0],
	    colstagger: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
	    dimensions: [10, 3],
	    keyheight: 0.5,
	    fingers: Fingermap {
		matrix: vec![vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
			     vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
			     vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP]],
		map: HashMap::new(),
            },
	};
	let sfb = Metric {
	    function: "sfb?".to_string(),
	    input: 2
	};
	
	let mut list = MetricList {
	    bigrams: HashMap::new(),
	    trigrams: HashMap::new(),
	};
	list.bigrams.insert("SFB".to_string(), sfb);
	let result = bake_metrics(&matrix, &interp, &list);
	println!("{:?}", result);
    }
}
