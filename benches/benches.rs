use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lib::config;
use lib::extractor::code;
use lib::extractor::git;
use lib::repo;
use lib::utils::test;
use std::time::Duration;

fn bench_git_extractor(c: &mut Criterion) {
    test::setup();
    c.bench_function("git extractor", |b| {
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
    test::teardown();
}

criterion_group! {
    name = git_extractor;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(500)).warm_up_time(Duration::from_secs(3));
    targets = bench_git_extractor
}

fn bench_code_extractor(c: &mut Criterion) {
    test::setup();
    c.bench_function("code extractor", |b| {
        b.iter(|| {
            let conf = config::Config::new();
            let storage_folder = format!("{}/{}", conf.wake_path, "repos");
            let url = "https://github.com/osscameroon/osscameroon-website".to_string();
            let r = match repo::new_repo_from_url(url, &storage_folder) {
                Ok(r) => r,
                Err(err) => panic!("{err:}"),
            };
            let path = code::get_repo_path(&r).unwrap();
            code::extract_code_data(path).unwrap();
        })
    });
    test::teardown();
}

criterion_group! {
    name = code_extractor;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(500)).warm_up_time(Duration::from_secs(3));
    targets = bench_code_extractor
}

criterion_main!(git_extractor, code_extractor);
