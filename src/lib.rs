use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

enum Message {
    SmallGroupResult(HashMap<i64, (i64, i64)>),
    LargeGroupResult(HashMap<i64, (i64, i64)>),
}

pub fn group_by_avg(pairs: &[(i64, i64)]) -> Vec<(i64, f64)> {
    if pairs.len() == 0 {
        return Vec::new();
    }

    let num = num_cpus::get().min(pairs.len());

    let large_groups = detect_large_groups(pairs);

    let small_group_plan_threads = num;
    let large_group_plan_threads = num;

    // the plan carries the payload for each thread
    let mut small_group_plan: Vec<Vec<(i64, i64)>> = iter::repeat(Vec::new())
        .take(small_group_plan_threads)
        .collect();
    let mut large_group_plan: Vec<Vec<(i64, i64)>> = iter::repeat(Vec::new())
        .take(large_group_plan_threads)
        .collect();

    let mut large_pair_count = 0;

    // plan the tasks
    for pair in pairs {
        if large_groups.contains(&pair.0) {
            // dispatch the pairs of large groups evenly into large group tasks
            large_group_plan[large_pair_count % large_group_plan_threads].push(*pair);
            large_pair_count += 1;
        } else {
            // dispatch the pairs of small groups by theirs keys,
            // this makes no key overlap between small group tasks
            small_group_plan[pair.0 as usize % small_group_plan_threads].push(*pair);
        }
    }

    // execute the plans
    let (tx, rx) = channel::<Message>();
    let pool = ThreadPool::new(num);

    for plan in small_group_plan {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(Message::SmallGroupResult(group_count_sum(&plan)))
                .unwrap();
        });
    }

    for plan in large_group_plan {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(Message::LargeGroupResult(group_count_sum(&plan)))
                .unwrap();
        });
    }

    // collect the results
    let mut collector: Vec<(i64, (i64, i64))> = Vec::new();
    let mut large_group_collector: HashMap<i64, (i64, i64)> = HashMap::new();

    for _ in 0..(small_group_plan_threads + large_group_plan_threads) {
        let msg = rx.recv().unwrap();
        match msg {
            Message::SmallGroupResult(map) => {
                // small group results can be collected directly
                // since there is no key overlap between tasks
                collector.extend(map.into_iter());
            }
            Message::LargeGroupResult(map) => {
                // merge results from large group tasks
                for (k, v) in map {
                    let &mut (ref mut count, ref mut sum) =
                        large_group_collector.entry(k).or_insert((0, 0));
                    *count += v.0;
                    *sum += v.1;
                }
            }
        }
    }

    collector.extend(large_group_collector.into_iter());

    // calculate the avg
    collector
        .into_iter()
        .map(|(k, (count, sum))| (k, sum as f64 / count as f64))
        .collect()
}

/// aggregate the pairs into (k) -> (count, sum)
fn group_count_sum(pairs: &[(i64, i64)]) -> HashMap<i64, (i64, i64)> {
    let mut map = HashMap::new();

    for (k, v) in pairs {
        let &mut (ref mut count, ref mut sum) = map.entry(*k).or_insert((0, 0));
        *count += 1;
        *sum += v;
    }

    map
}

const SAMPLES_COUNT: usize = 100;
const SAMPLES_THRESHOLD: usize = 5;

/// Detect the groups with large number of records
fn detect_large_groups(pairs: &[(i64, i64)]) -> Vec<i64> {
    let len = pairs.len();

    // if len < num_cpus::get() * 100 {
    //     return Vec::new();
    // }

    let mut rng = thread_rng();
    let mut samples: HashMap<i64, usize> = HashMap::new();

    for _ in 0..SAMPLES_COUNT {
        let sample = pairs[rng.gen_range(0, len)];
        *samples.entry(sample.0).or_insert(0) += 1;
    }

    let mut large_groups = Vec::new();

    for (k, v) in samples {
        if v >= SAMPLES_THRESHOLD {
            large_groups.push(k);
        }
    }

    large_groups
}

#[test]
fn test_basic() {
    let input = vec![];
    let result = group_by_avg(&input);
    let expected = vec![];
    assert_paris_equal(result, expected);

    let input = vec![(0, 0)];
    let result = group_by_avg(&input);
    let expected = vec![(0, 0.)];
    assert_paris_equal(result, expected);

    let input = vec![
        (0, 1),
        (1, 2),
        (0, 3),
        (2, 4),
        (7, 1),
        (3, 1),
        (0, 1),
        (3, 4),
        (6, 1),
        (3, 3),
        (0, 4),
        (3, 1),
        (4, 1),
    ];
    let result = group_by_avg(&input);
    let expected = vec![
        (0, 9. / 4.),
        (1, 2.),
        (2, 4.),
        (3, 9. / 4.),
        (4, 1.),
        (6, 1.),
        (7, 1.),
    ];
    assert_paris_equal(result, expected);
}

#[cfg(test)]
fn assert_paris_equal(a: Vec<(i64, f64)>, b: Vec<(i64, f64)>) {
    let map_a: HashMap<i64, f64> = a.into_iter().collect();
    let map_b: HashMap<i64, f64> = b.into_iter().collect();
    assert_eq!(map_a, map_b);
}
