use criterion::{black_box, criterion_group, criterion_main, Criterion};
use keynergy::analysis::{Analyzer, InputType, Metric, MetricList};
use keynergy::{fingers::*, Fingermap, Keyboard, Layout, TextData};
use std::collections::HashMap;

fn criterion_benchmark(c: &mut Criterion) {
    let mut metrics = MetricList::new();
    metrics.bigrams.insert(
        "SFB".to_string(),
        Metric {
            function: "sfb?".to_string(),
            input: InputType::Bigram,
        },
    );
    c.bench_function("TextData::from", |b| {
        b.iter(|| {
            TextData::from(black_box(
                "boat question saying trying, neighborhood".to_string(),
            ))
        })
    });
    let textdata = TextData::from("boat question saying trying, neighborhood".to_string());
    let mut analyzer = Analyzer::with(metrics, textdata);
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
    c.bench_function("calculate_metrics", |b| {
        b.iter(|| analyzer.calculate_metrics(black_box(&matrix)))
    });
    c.bench_function("analyze_keys", |b| {
        b.iter(|| {
            analyzer.analyze_keys(
                black_box(matrix.clone()),
                black_box(semimak.formats.standard.clone().unwrap()),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
