use reqwest::Client;
use tokio::runtime::Runtime;

fn main() {
  let rt = Runtime::new().unwrap();
  rt.block_on(async {
    let client = Client::new();

    get_report(&client).await.unwrap();
    list_rooms(&client).await.unwrap();

    create_room(&client, "testroom".to_string()).await.unwrap();
    list_rooms(&client).await.unwrap();

    delete_room(&client, "testroom".to_string()).await.unwrap();
    list_rooms(&client).await.unwrap();

    list_devices(&client, "kitchen1".to_string()).await.unwrap();

    create_device(
      &client,
      "kitchen1".to_string(),
      "socket".to_string(),
      "api_device".to_string(),
    )
    .await
    .unwrap();

    delete_device(&client, "kitchen1".to_string(), "api_device".to_string())
      .await
      .unwrap();
  })
}

async fn delete_device(
  client: &Client,
  room_name: String,
  device_name: String,
) -> reqwest::Result<()> {
  let resp = client
    .delete(format!(
      "http://localhost:3000/delete_device/{}/{}",
      room_name, device_name
    ))
    .send()
    .await?;
  println!("{:?}", resp.text().await.unwrap());
  Ok(())
}

async fn create_device(
  client: &Client,
  room_name: String,
  device_type: String,
  device_name: String,
) -> reqwest::Result<()> {
  let resp = client
    .post(format!(
      "http://localhost:3000/create_device/{}/{}/{}",
      room_name, device_type, device_name
    ))
    .send()
    .await?;
  println!("{:?}", resp.text().await.unwrap());
  Ok(())
}

async fn list_devices(client: &Client, room_name: String) -> reqwest::Result<()> {
  let resp = client
    .get(format!("http://localhost:3000/list_devices/{}", room_name))
    .send()
    .await?;
  println!("{:?}", resp.text().await.unwrap());
  Ok(())
}

async fn delete_room(client: &Client, room_name: String) -> reqwest::Result<()> {
  let resp = client
    .delete(format!("http://localhost:3000/delete_room/{}", room_name))
    .send()
    .await?;
  println!("{:?}", resp.text().await.unwrap());
  Ok(())
}

async fn create_room(client: &Client, room_name: String) -> reqwest::Result<()> {
  let resp = client
    .post(format!("http://localhost:3000/create_room/{}", room_name))
    .send()
    .await?;
  println!("{:?}", resp.text().await.unwrap());
  Ok(())
}

async fn get_report(client: &Client) -> reqwest::Result<()> {
  let resp = client.get("http://localhost:3000").send().await?;
  println!("{:?}", resp.text().await.unwrap());
  Ok(())
}

async fn list_rooms(client: &Client) -> reqwest::Result<()> {
  let resp = client
    .get("http://localhost:3000/list_rooms")
    .send()
    .await?;
  println!("{:?}", resp.text().await.unwrap());
  Ok(())
}
