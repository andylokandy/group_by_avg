use std::collections::HashMap;
use std::thread;

pub fn group_by_avg(pairs: &[(i64, i64)]) -> Vec<(i64, f64)> {
    let num = num_cpus::get();
    let num = if pairs.len() < num { pairs.len() } else { num };

    let handles: Vec<_> = (0..num)
        .map(|i| {
            // split chunks excuted in parallel
            let len = pairs.len();
            let chunk_size = len / num;
            let chunk = if i < num - 1 {
                Vec::from(&pairs[i * chunk_size..(i + 1) * chunk_size])
            } else {
                Vec::from(&pairs[i * chunk_size..])
            };

            thread::spawn(move || {
                let mut map: HashMap<i64, (i64, i64)> = HashMap::new();

                // group value into the form (k) -> (count, sum) for each chuncks
                for (k, v) in chunk {
                    let &mut (ref mut count, ref mut sum) = map.entry(k).or_default();
                    *count += 1;
                    *sum += v;
                }

                map
            })
        })
        .collect();

    let mut collecter: HashMap<i64, (i64, i64)> = HashMap::new();

    for handle in handles {
        let map = handle.join().expect("something went wrong");

        // merge results from all worker threads
        for (k, v) in map {
            let &mut (ref mut count, ref mut sum) = collecter.entry(k).or_default();
            *count += v.0;
            *sum += v.1;
        }
    }

    // calc the avg
    collecter
        .into_iter()
        .map(|(k, (count, sum))| (k, sum as f64 / count as f64))
        .collect()
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
