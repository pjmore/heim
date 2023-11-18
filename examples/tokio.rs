use futures::TryStreamExt;
use heim::sensors;

#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>> {
    let f = tokio::spawn(async move {
        sensors::temperatures()
            .try_collect::<Vec<sensors::TemperatureSensor>>()
            .await
        
            
    }).await??;
    println!("{:#?}", f);
    let f = tokio::spawn(async move {
        sensors::fans()
            .try_collect::<Vec<sensors::FanSensor>>()
            .await
            
    }).await??;
    println!("{:#?}", f);
    Ok(())
}