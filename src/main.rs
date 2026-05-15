use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::fs;
use std::env;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

async fn call_ollama(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let req = OllamaRequest {
        model: "qwen2.5-coder:7b-instruct".to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&req)
        .send()
        .await?
        .json::<OllamaResponse>()
        .await?;

    Ok(res.response)
}

fn split_into_n_prompts(prompt: &str, n: usize) -> Vec<String> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let n_parts: usize = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);

    println!("Will split the input prompt into {} sub-prompts.", n_parts);
    println!("Enter your multi-sentence prompt (Press Enter when finished):");
    
    let mut full_prompt = String::new();
    io::stdin().read_line(&mut full_prompt)?;

    if full_prompt.trim().is_empty() {
        println!("No prompt provided.");
        return Ok(());
    }

    let prompts = split_into_n_prompts(&full_prompt, n_parts);
    
    for (i, p) in prompts.iter().enumerate() {
        let part_num = i + 1;
        println!("\n--- Sub-prompt {} ---", part_num);
        println!("{}\n", p);

        let inst = fs::read_to_string("../instructions.txt").expect("Should read this file");  

        let pfull = p.to_owned() + &inst;

        println!("Feeding sub-prompt {} to Ollama...", part_num);
        println!("{}\n", pfull);
        let output = call_ollama(&pfull).await?;
        
        let filename = format!("out{}.txt", part_num);
        let mut file = File::create(&filename)?;
        write!(file, "{}", output)?;
        println!("Result saved to {}", filename);
    }

    Ok(())
}
