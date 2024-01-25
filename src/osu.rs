use reqwest::{header::HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};

const API_MAX_RETURNS: usize = 100;

pub async fn all_played_maps(
    client: &reqwest::Client,
    uid: i32,
) -> Result<Vec<PlayedMaps>, Box<dyn std::error::Error>> {
    let mut res: Vec<PlayedMaps> = Vec::new();

    let mut iteration: usize = 0;
    loop {
        let offest = iteration * API_MAX_RETURNS;
        let mut plays = played_maps(&client, uid, API_MAX_RETURNS, offest).await?;
        iteration = iteration + 1;
        if iteration % 5 == 0 {
            println!(
                "Finding Maps: Requested {} to {} and got {}",
                offest,
                offest + API_MAX_RETURNS,
                plays.len()
            );
        }

        if plays.len() < API_MAX_RETURNS {
            res.append(&mut plays);
            println!("Finding Maps: Reached the end of the plays");
            break;
        }

        res.append(&mut plays);
        std::thread::sleep(std::time::Duration::from_millis(100)); // Just in case
    }

    Ok(res)
}

async fn played_maps(
    c: &reqwest::Client,
    uid: i32,
    limit: usize,
    offset: usize,
) -> Result<Vec<PlayedMaps>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://osu.ppy.sh/users/{}/beatmapsets/most_played?limit={}&offset={}",
        uid, limit, offset
    );
    let resp: Vec<PlayedMaps> = c.get(url).send().await?.json().await?;
    Ok(resp)
}

pub async fn download_beatmap(
    client: &reqwest::Client,
    id: i64,
    cookies: &str,
    into: &str,
) -> Result<Option<()>, Box<dyn std::error::Error>> {
    let url = format!("https://osu.ppy.sh/beatmapsets/{}/download", id);
    let resp = client
        .get(url)
        .header("referer", format!("https://osu.ppy.sh/beatmapsets/{}", id))
        .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
        .header("cookie", cookies)
        .header(
            "ec-ch-ua",
            "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\"",
        )
        .header("sec-ch-ua-mobile", "?0")
        .send()
        .await?;
    let status = resp.status();
    if status == StatusCode::FOUND || status == StatusCode::OK {
        if match_header(resp.headers(), "accept-ranges", "bytes") {
            let mut file = tokio::fs::File::create(into).await?;
            let mut content = std::io::Cursor::new(resp.bytes().await?);
            tokio::io::copy(&mut content, &mut file).await?;
            Ok(Some(()))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn match_header(headers: &HeaderMap, header: &str, content: &str) -> bool {
    if let Some(h) = headers.get(header) {
        h.to_str().unwrap_or("") == content
    } else {
        false
    }
}

// Incomplete request so the server sends back
// proper cookies with the `set-cookie` header.
pub async fn get_user_cookies(
    client: &reqwest::Client,
    osu_session: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let resp = client
        .get("https://osu.ppy.sh/beatmapsets/41823/download")
        .header(
            "cookie",
            format!(
                "osu_session={}; path=/; domain=.ppy.sh; secure; httponly",
                osu_session
            ),
        )
        .header(
            "ec-ch-ua",
            "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\"",
        )
        .header("sec-ch-ua-mobile", "?0")
        .send()
        .await?;

    let cookies: Vec<&str> = resp
        .headers()
        .iter()
        .filter(|x| x.0.to_string() == "set-cookie")
        .map(|x| x.1.to_str().unwrap())
        .collect::<Vec<&str>>();

    Ok(cookies.join("; "))
}

// Auto-generated Structs:
use serde_json::Value;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayedMaps {
    #[serde(rename = "beatmap_id")]
    pub beatmap_id: i64,
    pub count: i64,
    pub beatmap: Beatmap,
    pub beatmapset: Beatmapset,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beatmap {
    #[serde(rename = "beatmapset_id")]
    pub beatmapset_id: i64,
    #[serde(rename = "difficulty_rating")]
    pub difficulty_rating: f64,
    pub id: i64,
    pub mode: String,
    pub status: String,
    #[serde(rename = "total_length")]
    pub total_length: i64,
    #[serde(rename = "user_id")]
    pub user_id: i64,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beatmapset {
    pub artist: String,
    #[serde(rename = "artist_unicode")]
    pub artist_unicode: String,
    pub covers: Covers,
    pub creator: String,
    #[serde(rename = "favourite_count")]
    pub favourite_count: i64,
    pub hype: Value,
    pub id: i64,
    pub nsfw: bool,
    pub offset: i64,
    #[serde(rename = "play_count")]
    pub play_count: i64,
    #[serde(rename = "preview_url")]
    pub preview_url: String,
    pub source: String,
    pub spotlight: bool,
    pub status: String,
    pub title: String,
    #[serde(rename = "title_unicode")]
    pub title_unicode: String,
    #[serde(rename = "track_id")]
    pub track_id: Option<i64>,
    #[serde(rename = "user_id")]
    pub user_id: i64,
    pub video: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Covers {
    pub cover: String,
    #[serde(rename = "cover@2x")]
    pub cover_2x: String,
    pub card: String,
    #[serde(rename = "card@2x")]
    pub card_2x: String,
    pub list: String,
    #[serde(rename = "list@2x")]
    pub list_2x: String,
    pub slimcover: String,
    #[serde(rename = "slimcover@2x")]
    pub slimcover_2x: String,
}
