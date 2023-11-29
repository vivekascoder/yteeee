use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
struct Json3SubSeg {
    utf8: String,
}

#[derive(Serialize, Debug, Deserialize)]
struct Json3SubEvent {
    segs: Option<Vec<Json3SubSeg>>,
}

#[derive(Serialize, Debug, Deserialize)]
struct Json3Subtitle {
    events: Vec<Json3SubEvent>,
}

impl Json3Subtitle {
    pub fn to_string(&self) -> String {
        let mut text = "".to_string();
        for eve in &self.events {
            if eve.segs.is_none() {
                continue;
            }
            text.push_str(
                eve.segs
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|s| s.utf8.to_string())
                    .collect::<Vec<String>>()
                    .join("")
                    .as_str(),
            )
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::Json3Subtitle;

    #[test]
    fn test_json3_to_str() {
        let json3_text = std::fs::read_to_string("./static/json3_sub.json").unwrap();
        let json3: Json3Subtitle = serde_json::from_str(json3_text.as_str()).unwrap();
        println!("text subtitle: \n\n{}", json3.to_string());
    }
}
