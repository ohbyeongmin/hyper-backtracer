use tokio::time::{sleep, Duration};

pub struct DbManager {
    pub name: String,
}

impl DbManager {
    pub async fn write(&self, input: &str) -> String {
        println!("DB {} getting : {input}", self.name);
        println!("writing db...: {input}");
        sleep(Duration::from_secs(2)).await;
        format!("{input} is done")
    }
}
