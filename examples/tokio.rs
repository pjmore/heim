use futures::StreamExt;
use heim::sensors;

#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>> {
    let f = tokio::spawn(async move {
        sensors::temperatures()
            .collect::<Vec<Result<sensors::TemperatureSensor, heim::Error>>>()
            .await
            
    }).await?;
    println!("{:#?}", f);
    Ok(())
}