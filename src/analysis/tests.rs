use crate::{analysis::classify_ngram, fingers::*, Fingermap};

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
            matrix: vec![
                vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
                vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
                vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
            ],
            map: HashMap::new(),
        },
    };
    let sfb = Metric {
        function: "sfb?".to_string(),
        input: 2,
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
            matrix: vec![
                vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
                vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
                vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
            ],
            map: HashMap::new(),
        },
    };
    let sfb = Metric {
        function: "sfb?".to_string(),
        input: 2,
    };

    let mut list = MetricList {
        bigrams: HashMap::new(),
        trigrams: HashMap::new(),
    };
    list.bigrams.insert("SFB".to_string(), sfb);
    let result = &matrix.bake_metrics(&interp, &list);
    println!("{:?}", result);
}
