pub mod analyzer;
pub mod combinedpos;
pub mod error;
pub mod metrics;
pub use analyzer::Analyzer;
pub use combinedpos::{CombinedPos, CombinedPosGroup};
pub use error::{AnalysisError, ErrorType};
pub use metrics::{InputType, Metric, MetricAmount, MetricList, MetricMap, MetricTotal};

#[cfg(test)]
mod tests {
    use crate::{
        analysis::{Analyzer, InputType, Metric, MetricList, MetricTotal},
        fingers::*,
        Fingermap, Keyboard, Layout, TextData,
    };
    use ketos::FromValueRef;
    use std::collections::HashMap;
    #[test]
    fn classify() {
        let mut metrics = MetricList::new();
        metrics.bigrams.insert(
            "SFB".to_string(),
            Metric {
                function: "sfb?".to_string(),
                input: InputType::Bigram,
            },
        );
        let mut analyzer = Analyzer::with(metrics, TextData::from("queuestion".to_string()));
        analyzer
            .interpreter
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

        let semimak = Layout::load("testdata/semimak_jq.toml").unwrap();
        analyzer.calculate_metrics(&matrix);
        println!("{:?}", analyzer.keyboard_stats);
        let result = analyzer.analyze_keys(matrix, semimak.formats.standard.unwrap());
        assert_eq!(*result.unwrap().get("SFB").unwrap(), MetricTotal::Count(2));
    }
}
