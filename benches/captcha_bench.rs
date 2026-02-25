use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use captcha_rs::CaptchaBuilder;

fn bench_captcha_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Captcha Generation");

    group.bench_function("default", |b| {
        b.iter(|| {
            CaptchaBuilder::new()
                .length(5)
                .width(130)
                .height(40)
                .build();
        });
    });

    group.bench_function("high_complexity", |b| {
        b.iter(|| {
            CaptchaBuilder::new()
                .length(5)
                .width(200)
                .height(70)
                .complexity(10) // Max complexity (noise)
                .build();
        });
    });

    group.bench_function("high_distortion", |b| {
        b.iter(|| {
            CaptchaBuilder::new()
                .length(5)
                .width(200)
                .height(70)
                .distortion(15) // High wavy distortion
                .build();
        });
    });

    group.bench_function("extreme_all", |b| {
        b.iter(|| {
            CaptchaBuilder::new()
                .length(8)
                .width(300)
                .height(100)
                .complexity(10)
                .distortion(20)
                .drop_shadow(true)
                .interference_lines(10)
                .interference_ellipses(10)
                .build();
        });
    });

    group.finish();
}

criterion_group!(benches, bench_captcha_generation);
criterion_main!(benches);
