use serde::{Serialize, Deserialize};
use reqwest;
// use reqwest::{Error, Response};
use serde_json::Value;
use rand::prelude::*;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
struct Attachment {
  title: String,
  image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackMessage {
  response_type: String,
  channel: String,
  attachments: [Attachment; 1],
}

#[derive(Serialize, Deserialize)]
pub struct SlackRequest {
  token: String,
  team_id: String,
  team_domain: String,
  channel_id: String,
  channel_name: String,
  thread_ts: String,
  timestamp: String,
  user_id: String,
  user_name: String,
  text: String,
  trigger_word: String,
}

fn find_good_url(children: &Value, index: usize, max: usize, start: usize) -> String {
  let url = children[index]["data"]["url"].to_string().replace("\"", "");
  let copied_url = url.clone();
  let pattern = Regex::new(r"(\.gif|\.jpg|\.png|\.bmp)\b").unwrap();

  let image = match pattern.captures(&copied_url) {
    Some(_) => true,
    None => false
  };

  // I am way too lazy to exhaustively check all 10
  if image {
    url
  } else if index == 0 {
    find_good_url(children, max, max, start)
  }else if index == start + 1{
    "http://i.imgur.com/5qMAsSS.gif".to_string()
  } else {
    find_good_url(children, index-1, max, start)
  }
}

fn parse_response(body: Value) -> Result<String, String> {
  let mut rng = thread_rng();
  let index = rng.gen_range(0, 9);

  let children = &body["data"]["children"];

  let url = find_good_url(children, index, 10, index);

  if url.is_empty() || url == "null" {
    Err("Error Parsing JSON".to_string())
  } else {
    Ok(url)
  }
}

fn make_slack_response(url: String) -> SlackMessage {
  let attachment = Attachment {
    title: "Don't panic! Here is a cute picture to soothe you.".to_string(),
    image_url: url
  };

  SlackMessage {
    response_type: "ephemeral".to_string(),
    channel: "#general".to_string(),
    attachments: [attachment],
  }
}

pub fn get_top_aww_post(url: &str) -> Result<SlackMessage, String> {
  let body: Value = reqwest::get(url).unwrap().json().unwrap();
  let slack_message = match parse_response(body) {
    Ok(response) => {
      make_slack_response(response)
    },
    Err(e) => return Err(e)
  };
  println!("{:?}", slack_message);
  Ok(slack_message)
}
