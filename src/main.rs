use serde::Deserialize;

  #[derive(Deserialize, Debug)]
  struct Account {
      puuid: String,
      #[serde(rename = "gameName")]
      game_name: String,
      #[serde(rename = "tagLine")]
      tag_line: String,
  }

  #[tokio::main]
  async fn main() {
      dotenvy::dotenv().ok();
      let api_key = std::env::var("RIOT_API_KEY").expect("RIOT_API_KEY not
  set");

      let game_name = "Domis";  // replace with your Riot name
      let tag_line = "2003";        // replace with your tagline

      let url = format!(
          "https://europe.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
          game_name, tag_line
      );

      let client = reqwest::Client::new();
      println!("{}", url);
      let response = client
          .get(&url)
          .header("X-Riot-Token", &api_key)
          .send()
          .await
          .expect("request failed");
        
 let account: Account = response.json().await.expect("failed to parse response");

      println!("{:#?}", account);

  }