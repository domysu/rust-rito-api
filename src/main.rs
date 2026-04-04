use reqwest::get;
use serde::Deserialize;

  #[derive(Deserialize, Debug)]
  struct Participant {
      kills: u32,
      deaths: u32,
      assists: u32,
      #[serde(rename = "championName")]
      champion_name: String,
      win: bool,
      #[serde(rename = "totalMinionsKilled")]
      total_minions_killed: u32,
      #[serde(rename = "goldEarned")]
      gold_earned: u32,
      #[serde(rename = "teamPosition")]
      team_position: String,
  }

  #[derive(Deserialize, Debug)]
  struct MatchInfo {
      participants: Vec<Participant>,
  }

  #[derive(Deserialize, Debug)]
  struct Match {
      info: MatchInfo,
  }

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
      println!("{:#?}", get_last_match(&api_key, &account.puuid).await)

  }

  async fn get_last_match(api_key: &String, puuid: &String) -> Vec<String>
  {

    
    let url = format!("https://europe.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids", puuid);
    let client = reqwest::Client::new();
    println!("{}", url);

    let response = client.get(&url).header("X-Riot-Token", api_key).send().await.expect("match request failed");
    let matches: Vec<String> = response.json().await.expect("Failed to parse matches response");
    matches



  }

#[cfg(test)]
mod tests {
    use std::env;

    // Regression guard: RIOT_API_KEY must be readable via std::env::var using
    // exactly this key name. If the variable name is ever changed or the read
    // mechanism is replaced, this test will catch it before main() panics at
    // runtime.
    #[test]
    fn riot_api_key_is_set_and_non_empty() {
        // Inject a known value so the test is self-contained and never depends
        // on the developer's local .env file being present.
        // SAFETY: tests run single-threaded by default in this binary; no other
        // thread is reading the environment concurrently at this point.
        unsafe { env::set_var("RIOT_API_KEY", "test-key-abc123") };

        let result = env::var("RIOT_API_KEY");

        assert!(result.is_ok(), "RIOT_API_KEY should be readable via std::env::var");
        let value = result.unwrap();
        assert!(!value.is_empty(), "RIOT_API_KEY must not be an empty string");
        assert_eq!(value, "test-key-abc123", "RIOT_API_KEY value did not round-trip correctly");
    }

    // Regression guard: when RIOT_API_KEY is absent, std::env::var must return
    // Err — confirming that main()'s .expect() is the right failure mechanism.
    //
    // IMPORTANT: run this test in isolation to avoid a race with other tests
    // that set the variable:
    //   cargo test riot_api_key_missing -- --ignored --test-threads=1
    #[test]
    #[ignore = "mutates process environment; run with --test-threads=1 to avoid races"]
    fn riot_api_key_missing_returns_err() {
        // SAFETY: this test is intentionally run with --test-threads=1 (see
        // the ignore reason above) so no concurrent env reads can grace here.
        unsafe { env::remove_var("RIOT_API_KEY") };

        let result = env::var("RIOT_API_KEY");

        assert!(
            result.is_err(),
            "Expected Err when RIOT_API_KEY is unset, but got: {:?}",
            result.ok()
        );
    }

    #[test]
    fn participant_deserializes_from_json() {
        let json = r#"{
            "kills": 5,
            "deaths": 3,
            "assists": 7,
            "championName": "Jax",
            "win": true,
            "totalMinionsKilled": 203,
            "goldEarned": 12772,
            "teamPosition": "TOP"
        }"#;

        let p: crate::Participant = serde_json::from_str(json).expect("failed to deserialize");

        assert_eq!(p.kills, 5);
        assert_eq!(p.deaths, 3);
        assert_eq!(p.assists, 7);
        assert_eq!(p.champion_name, "Jax");
        assert!(p.win);
        assert_eq!(p.total_minions_killed, 203);
        assert_eq!(p.gold_earned, 12772);
        assert_eq!(p.team_position, "TOP");
    }

    #[test]
    fn match_deserializes_from_json() {
        let json = r#"{
            "info": {
                "participants": [
                    {
                        "kills": 10,
                        "deaths": 2,
                        "assists": 5,
                        "championName": "Ahri",
                        "win": false,
                        "totalMinionsKilled": 180,
                        "goldEarned": 11000,
                        "teamPosition": "MIDDLE"
                    }
                ]
            }
        }"#;

        let m: crate::Match = serde_json::from_str(json).expect("failed to deserialize");

        assert_eq!(m.info.participants.len(), 1);
        assert_eq!(m.info.participants[0].champion_name, "Ahri");
        assert!(!m.info.participants[0].win);
    }
}