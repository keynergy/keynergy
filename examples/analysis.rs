//! This example shows how to perform basic layout analysis

use keynergy::{
    analysis::{Analyzer, InputType, Metric, MetricList, MetricTotal},
    Keyboard, Keys, TextData,
};

const DEF_SFB: &'static str = r#"
    (define (sfb? a b)
        (= (finger a) (finger b)))
    "#;

const SAMPLE_TEXT: &'static str = "decided";

fn analyzer(text_data: &TextData) -> Analyzer<'_> {
    // Choose metrics you are interested in
    let mut metrics = MetricList::new();
    metrics.bigrams.insert(
        "SFB".to_string(),
        Metric {
            function: "sfb?".to_string(),
            input: InputType::Bigram,
        },
    );
    let analyzer = Analyzer::with(metrics, &text_data);
    // Define chosen metrics
    analyzer.interpreter.run_code(DEF_SFB, None).unwrap();
    analyzer
}

fn main() {
    // Calculate stats of a chosen corpus
    let text_data = TextData::from(SAMPLE_TEXT);

    let mut analyzer = analyzer(&text_data);

    // Choose physical and logical layouts
    let matrix_kb = Keyboard::matrix();
    let qwerty = Keys::qwerty();

    // Bake in metrics for physical layout
    analyzer.calculate_metrics(&matrix_kb).unwrap();
    // And analyze logical layout
    let result = analyzer.analyze_keys(&matrix_kb, &qwerty).unwrap();

    println!("{:?}", result);
    assert_eq!(*result.get("SFB").unwrap(), MetricTotal::Count(4));
}
