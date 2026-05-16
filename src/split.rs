
pub fn split_into_n_prompts(prompt: &str, n: usize) -> Vec<String> {
    if n <= 1 {
        return vec![prompt.trim().to_string()];
    }

    let period_indices: Vec<usize> = prompt.char_indices()
        .filter(|&(_, c)| c == '.')
        .map(|(i, _)| i)
        .collect();

    if period_indices.is_empty() {
        let chunk_size = prompt.len() / n;
        let mut parts = Vec::new();
        let mut start = 0;
        for i in 1..n {
            let end = i * chunk_size;
            parts.push(prompt[start..end].trim().to_string());
            start = end;
        }
        parts.push(prompt[start..].trim().to_string());
        return parts;
    }

    let mut split_points = Vec::new();
    let total_len = prompt.len();

    for i in 1..n {
        let target = (i * total_len) / n;
        let best_period = period_indices.iter()
            .min_by_key(|&&idx| if idx > target { idx - target } else { target - idx })
            .unwrap();
        split_points.push(*best_period);
    }
    
    split_points.sort();
    split_points.dedup();

    let mut result = Vec::new();
    let mut last_idx = 0;
    for &idx in &split_points {
        result.push(prompt[last_idx..=idx].trim().to_string());
        last_idx = idx + 1;
    }
    result.push(prompt[last_idx..].trim().to_string());

    result

    }

