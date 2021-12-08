use anyhow::Result;
use std::{
    path::Path,
    time::{Duration, Instant},
};

const MINIMUM_TIME_BETWEEN_DOWNLOADS: Duration = Duration::from_secs(3);

#[derive(Debug, Default)]
pub struct Inputs {
    session_key: Option<String>,
    last_download_time: Option<Instant>,
}

impl Inputs {
    pub fn new() -> Inputs {
        Default::default()
    }

    pub fn get(&mut self, day: u32) -> Result<String> {
        let path = format!("./inputs/{day:0>2}.txt");
        let path = Path::new(&path);
        if let Ok(input) = std::fs::read_to_string(&path) {
            return Ok(input.replace("\r\n", "\n"));
        }

        let input = self.download(day)?;
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, &input)?;
        Ok(input)
    }

    fn get_session_key(&mut self) -> Result<&str> {
        if self.session_key.is_none() {
            self.session_key = Some(std::fs::read_to_string("./session_key.txt")?);
        }
        Ok(self.session_key.as_ref().unwrap())
    }

    fn download(&mut self, day: u32) -> Result<String> {
        let session_key = self.get_session_key()?;
        let cookie_values = format!("session={session_key}");

        let current_time = Instant::now();
        if let Some(last_time) = self.last_download_time {
            let delta_time = current_time - last_time;
            if delta_time < MINIMUM_TIME_BETWEEN_DOWNLOADS {
                std::thread::sleep(delta_time - MINIMUM_TIME_BETWEEN_DOWNLOADS);
            }
        }
        self.last_download_time = Some(current_time);

        let resp = ureq::get(&format!("https://adventofcode.com/2021/day/{day}/input"))
            .set("cookie", &cookie_values)
            .timeout(Duration::from_secs(5))
            .call()?;

        Ok(resp.into_string()?)
    }
}
