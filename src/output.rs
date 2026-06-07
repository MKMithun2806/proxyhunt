use std::fs::File;
use std::io::{Write, BufWriter};
use anyhow::Result;
use crate::checker::CheckedProxy;
use tracing::info;

pub fn save_results(results: &[CheckedProxy], output_path: &str, json_path: Option<&str>, limit: Option<usize>) -> Result<()> {
    let to_save = if let Some(l) = limit {
        if results.len() > l {
            &results[..l]
        } else {
            results
        }
    } else {
        results
    };

    // Save TXT
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    for r in to_save {
        writeln!(writer, "{}", r.proxy.full_url())?;
    }
    writer.flush()?;
    info!("Saved {} proxies to {}", to_save.len(), output_path);

    // Save JSON
    if let Some(jp) = json_path {
        let file = File::create(jp)?;
        serde_json::to_writer_pretty(file, to_save)?;
        info!("Saved {} proxies to {}", to_save.len(), jp);
    }

    Ok(())
}
