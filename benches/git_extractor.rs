use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lib::config;
use lib::extractor::git;
use lib::repo;
use lib::utils::test;
use std::time::Duration;

fn bench_extractor(c: &mut Criterion) {
    test::setup();

    let mut group = c.benchmark_group("extractor");
    // Measure the performance of the extract_git_objects function
    group.bench_function("git data extraction", |b| {
        b.iter(|| {
            let conf = config::Config::new();
            let storage_folder = format!("{}/{}", conf.wake_path, "repos");
            let url = "https://github.com/osscameroon/osscameroon-website".to_string();
            let r = match repo::new_repo_from_url(url, &storage_folder) {
                Ok(r) => r,
                Err(err) => panic!("{err:}"),
            };
            git::extract_git_objects(black_box(&r)).unwrap();
        })
    });

    // Measure the performance of the par_extract_git_objects function
    group.bench_function("git parallel data extraction", |b| {
        b.iter(|| {
            let conf = config::Config::new();
            let storage_folder = format!("{}/{}", conf.wake_path, "repos");
            let url = "https://github.com/osscameroon/osscameroon-website".to_string();
            let r = match repo::new_repo_from_url(url, &storage_folder) {
                Ok(r) => r,
                Err(err) => panic!("{err:}"),
            };
            git::par_extract_git_objects(black_box(&r)).unwrap();
        })
    });

    test::teardown();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(500)).warm_up_time(Duration::from_secs(3));
    targets = bench_extractor
}
criterion_main!(benches);
