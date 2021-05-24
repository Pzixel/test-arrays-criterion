use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::Rng;
use rand::distributions::Uniform;
use itertools::Itertools;

const TOTAL_NUMS: usize = 100000;
const SAMPLES: usize = 500;
const SEARCHES_COUNT: usize = 10;

fn binary_search(nums: &[i32], searches: &[i32]) -> i32 {
    let mut counter = 0;
    for i in searches.iter() {
        if let Ok(_) = nums.binary_search(&i) {
            counter += 1;
        };
    }
    return counter;
}

fn linear_search(nums: &[i32], searches: &[i32]) -> i32 {
    let mut counter = 0;
    for i in searches.iter() {
        for x in nums.iter() {
            if x == i {
                counter += 1;
            };
        }
    }
    return counter;
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Array search");
    let arrs: Vec<_> = (0..SAMPLES).map(|_| {
        let rng = rand::thread_rng();
        rng.sample_iter(Uniform::new(i32::min_value(), i32::max_value())).take(TOTAL_NUMS).collect::<Vec<_>>()
    }).collect();
    let mut rng = rand::thread_rng();
    for (i, nums) in arrs.iter().enumerate() {
        let sorted_nums: Vec<_> = nums.iter().sorted().cloned().collect();
        let searches: Vec<_> = (0..SEARCHES_COUNT).map(|_| nums[rng.gen_range(0..nums.len())]).collect();

        let nums = nums.clone().into_boxed_slice();
        let sorted_nums = sorted_nums.into_boxed_slice();
        let searches = searches.into_boxed_slice();

        group.bench_with_input(BenchmarkId::new("Binary", i), &(sorted_nums.clone(), searches.clone()),
                               |b, (nums, searches)| b.iter(|| binary_search(&nums, searches.as_ref())));
        group.bench_with_input(BenchmarkId::new("Sorted", i), &(sorted_nums.clone(), searches.clone()),
                               |b, (nums, searches)| b.iter(|| linear_search(&nums, searches.as_ref())));
        group.bench_with_input(BenchmarkId::new("Unsorted", i), &(nums.clone(), searches.clone()),
                               |b, (nums, searches)| b.iter(|| linear_search(&nums, searches.as_ref())));
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
