use criterion::{black_box, criterion_group, criterion_main, Criterion};
use keynergy::analysis::{Analyzer, InputType, Metric, MetricList};
use keynergy::{fingers::*, Keyboard, Layout, TextData};

const TEXT: &'static str = "boat question saying trying, neighborhood";

fn matrix() -> Keyboard {
    Keyboard {
        name: "Matrix".to_string(),
        rowstagger: vec![0.0, 0.0, 0.0],
        colstagger: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        dimensions: [10, 3],
        keyheight: 0.5,
        fingers: vec![
            vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
            vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
            vec![LP, LR, LM, LI, LI, RI, RI, RM, RR, RP],
        ],
    }
}

fn analyzer(textdata: &TextData) -> Analyzer<'_> {
    let mut metrics = MetricList::new();
    metrics.bigrams.insert(
        "SFB".to_string(),
        Metric {
            function: "sfb?".to_string(),
            input: InputType::Bigram,
        },
    );
    let analyzer = Analyzer::with(metrics, &textdata);
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
    analyzer
}

fn bench_text_data(c: &mut Criterion) {
    c.bench_function("TextData::from", |b| {
        b.iter(|| {
            TextData::from(black_box(TEXT))
        })
    });
}

fn bench_calculate_metrics(c: &mut Criterion) {
    let textdata = TextData::from(TEXT);
    let mut analyzer = analyzer(&textdata);
    let matrix = matrix();
    c.bench_function("calculate_metrics", |b| {
        b.iter(|| analyzer.calculate_metrics(black_box(&matrix)))
    });
}

fn bench_analyze_keys(c: &mut Criterion) {
    let textdata = TextData::from(TEXT);
    let analyzer = analyzer(&textdata);
    let matrix = matrix();
    let semimak = Layout::load("testdata/semimak_jq.toml").unwrap();
    let keys = semimak.formats.standard.as_ref().unwrap();
    c.bench_function("analyze_keys", |b| {
        b.iter(|| {
            analyzer.analyze_keys(
                black_box(&matrix),
                black_box(&keys),
            )
        })
    });
}

fn bench(c: &mut Criterion) {
    bench_text_data(c);
    bench_calculate_metrics(c);
    bench_analyze_keys(c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
