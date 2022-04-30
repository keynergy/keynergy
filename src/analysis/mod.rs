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
        Keyboard, Keys, TextData,
    };
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
        let td = TextData::from("look under");
        let mut analyzer = Analyzer::with(metrics, &td);
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

        let matrix = Keyboard::matrix();

        let qwerty = Keys::qwerty();
        analyzer.calculate_metrics(&matrix);
        let result = analyzer.analyze_keys(matrix, qwerty);
        assert_eq!(*result.unwrap().get("SFB").unwrap(), MetricTotal::Count(4));
    }
}
