use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub id: String,
        pub model: String,
            pub prompt: String,
            }

            pub async fn get_next_job() -> Job {
                Job {
                        id: "manual-test".into(),
                                model: "lb".into(),
                                        prompt: "Hello world".into(),
                                            }
                                            }