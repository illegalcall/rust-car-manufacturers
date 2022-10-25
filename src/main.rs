#![deny(clippy::all)]

use std::env;
const API_URL:&str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";

#[derive(Debug)]
struct Manufacturer<'a>{
    name: Option<&'a str>,
    common_name: Option<&'a str>,
    country: Option<&'a str>
}

trait Contains{
    fn contains(&self, needle:&str) -> bool;
}

impl<'a> Contains for Manufacturer<'a>{
    fn contains(&self, needle:&str) -> bool {
        self.name.unwrap_or_default().to_lowercase().contains(&needle.to_lowercase()) ||
          self.common_name.unwrap_or_default().to_lowercase().contains(&needle.to_lowercase()) ||  
          self.country.unwrap_or_default().to_lowercase().contains(&needle.to_lowercase())
    }
}

impl<'a> Manufacturer<'a>{
    fn description(&self) -> String{
        let name = self.name.unwrap_or_default();
        let common_name = self.common_name.unwrap_or_default();
        let country = self.country.unwrap_or_default();

        format!("\tName: {name}\n\tCommon Name:{common_name}\n\tCountry:{country}")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //Get input
    let args:Vec<String> = env::args().collect();
    //Validate input
    if args.len() <2{
        println!("Usage:{}", args[0]);
        return Ok(());
    }

    let keyword = &args[1];

    //Client setup
    let client = reqwest::Client::new();
    //Getting the API Response
    let res = client.get(API_URL).send().await?.json::<serde_json::Value>().await?;

    //Get Manufacturers from the api response
    let manufacturers_json = res.as_object().unwrap().get("Results").unwrap().as_array().unwrap().iter();
    
    //Store in a vector of Manufacturer structs
    let manufacturers = manufacturers_json.map(|m| {
        let obj = m.as_object().unwrap();
        let name = obj.get("Mfr_Name").unwrap().as_str();
        let common_name = obj.get("Mfr_CommonName").unwrap().as_str();
        let country = obj.get("Country").unwrap().as_str();

        Manufacturer{
            name,
            common_name,
            country
        }
    });
    
    //Search manufacturers vec for matching keyword
    let found_manufacturers = manufacturers.filter(|m| m.contains(keyword)).collect::<Vec<_>>();

    if found_manufacturers.is_empty(){
       return Err("No manufacturers found".into());
    } else{
        for (index,manu) in found_manufacturers.iter().enumerate(){
            println!("Manufacturer #{}",index+1);
            println!("{}", manu.description());
        }
    }

    Ok(())
}
