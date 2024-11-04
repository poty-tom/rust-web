use reqwest::Error;

async fn get_request() -> Result<(), Error> {
    let res = reqwest::get("https://www.fruityvice.com/api/fruit/apple").await?;
    println!("Status: {}", res.status());

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    get_request().await?;
    Ok(())
}
