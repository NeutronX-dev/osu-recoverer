pub mod osu;
use std::{collections::HashMap, hash::Hash, io::Write, os::windows::fs::MetadataExt};

use osu::{get_user_cookies, PlayedMaps};

fn ask(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut res = String::new();

    std::io::stdin().read_line(&mut res)?;
    res = String::from(res.trim());

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uid: i32 = ask("osu!user_id: ")?.parse()?;
    let session = ask("osu!session: ")?;
    let mut delay: u64 = 975;

    let client = reqwest::Client::new();
    let maps = osu::all_played_maps(&client, uid).await?;
    println!("Total played: {} maps", maps.len());

    let user_cookies = get_user_cookies(&client, &session).await?;

    // Turn into a HashMap with the beatmapset id as a key rather than the beatmap id to prevent
    // downloading multiple diffs of the same map.
    let maps = vec_to_hashmap(maps, |map| map.beatmapset.id);
    // Recompile the Data
    let mut maps = maps
        .iter()
        .map(|(_x, y)| y.to_owned())
        .collect::<Vec<PlayedMaps>>();

    maps.sort_by(|a, b| a.count.partial_cmp(&b.count).unwrap());
    println!(
        "Removed beatmapset duplicates; Downloading {} maps...",
        maps.len()
    );
    let mut attempts = 0;
    while maps.len() > 0 {
        let map = maps.first().unwrap().to_owned();
        if attempts == 3 {
            maps.remove(0);
            attempts = 0;
            continue;
        }

        let path = format!("./maps/{}.osz", map.beatmapset.id);

        if path_exists(&path) && attempts == 0 {
            maps.remove(0);
            println!(
                "Download: ❌ : {} - {} (File already exists)",
                map.beatmapset.artist, map.beatmapset.title
            );
            continue;
        }

        std::thread::sleep(std::time::Duration::from_millis(delay)); // Prevent Rate Limit

        match osu::download_beatmap(&client, map.beatmapset.id, &user_cookies, &path).await {
            Ok(success) => {
                if success.is_none() {
                    attempts = attempts + 1;
                    delay = delay + 25;
                    println!(
                        "Download: ❌ : {} - {} ({}/3 attempts) (API No Return)",
                        map.beatmapset.artist, map.beatmapset.title, attempts
                    );
                    continue;
                }
                attempts = 0;
                maps.remove(0);
                println!(
                    "Download: ✔ : {} - {}",
                    map.beatmapset.artist, map.beatmapset.title
                );
                continue;
            }
            Err(_err) => {
                println!(
                    "Download: ❌ : {} - {} ({}/3 attempts) (Request Fail)",
                    map.beatmapset.artist, map.beatmapset.title, attempts
                );
                attempts = attempts + 1;
                delay = delay + 25;
                continue;
            }
        }
    }

    let _ = ask("All done, press [Enter] to close");
    Ok(())
}

pub fn vec_to_hashmap<V, K, F>(v: Vec<V>, f: F) -> HashMap<K, V>
where
    F: Fn(&V) -> K,
    K: Eq + PartialEq + Hash,
{
    let mut res: HashMap<K, V> = HashMap::new();
    for item in v {
        res.insert(f(&item), item);
    }

    res
}

pub fn path_exists(path: &str) -> bool {
    let metadata = std::fs::metadata(path);
    if metadata.is_ok() {
        std::fs::metadata(path).unwrap().file_size() > 0
    } else {
        false
    }
}
