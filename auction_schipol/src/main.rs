use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;
use csv::Writer;
use anyhow::{Result, Context};

#[derive(Debug, Deserialize, Serialize)]
struct AuctionItem {
    Id: i64,
    AuctioneerID: String,
    Auction: String,
    AuctSessionID: i64,
    AuctSessionName: String,
    GoedID: i64,
    Lotnr: String,
    Description: String,
    LowEstimate: String,
    HighEstimate: String,
    Search: String,
    ImageURL: String,
    datumTot: String,
    LowEstimateNum: i64,
}

fn main() -> Result<()> {
    // Specify the directory path where JSON files are located
    let dir_path = "/Users/sramzzs4d/Projects-sramzz/rustTraining/auction_schipol/auction_json"; // Update this path
    
    // Get all JSON files in the directory
    let mut all_items = Vec::new();
    
    for entry in fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read directory: {}", dir_path))? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            // Read and parse JSON file
            let content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read file: {}", path.display()))?;
            
            let items: Vec<AuctionItem> = serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse JSON from file: {}", path.display()))?;
            
            all_items.extend(items);
        }
    }
    
    // Save as JSON
    let json_output = serde_json::to_string_pretty(&all_items)?;
    let mut json_file = File::create(Path::new(dir_path).join("combined_output.json"))?;
    json_file.write_all(json_output.as_bytes())?;
    
    // Save as CSV
    let mut csv_writer = Writer::from_path(Path::new(dir_path).join("output.csv"))?;
    
    // Write CSV headers
    csv_writer.write_record(&[
        "Id",
        "AuctioneerID",
        "Auction",
        "AuctSessionID",
        "AuctSessionName",
        "GoedID",
        "Lotnr",
        "Description",
        "LowEstimate",
        "HighEstimate",
        "Search",
        "ImageURL",
        "datumTot",
        "LowEstimateNum",
    ])?;
    
    // Write data rows
    for item in all_items {
        csv_writer.write_record(&[
            item.Id.to_string(),
            item.AuctioneerID,
            item.Auction,
            item.AuctSessionID.to_string(),
            item.AuctSessionName,
            item.GoedID.to_string(),
            item.Lotnr,
            item.Description,
            item.LowEstimate,
            item.HighEstimate,
            item.Search,
            item.ImageURL,
            item.datumTot,
            item.LowEstimateNum.to_string(),
        ])?;
    }
    
    csv_writer.flush()?;
    
    println!("Processing completed successfully!");
    Ok(())
}