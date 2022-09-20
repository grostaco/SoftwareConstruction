pub struct Assignment {
    pub number: u64,
    pub about: Option<String>,
}

pub async fn get_assignments() -> Vec<Assignment> {
    let mut assignments = Vec::new();
    let mut i = 1;
    loop {
        let base = format!("https://grostaco.github.io/SoftwareConstruction/assignment{i}/");
        let res = reqwest::get(base.clone() + "index.html").await.unwrap();
        if res.status().as_u16() == 200 {
            // TODO: not.. do this
            let about = match reqwest::get(base.clone() + "about.txt")
                .await
                .map(|res| async {
                    res.text()
                        .await
                        .unwrap_or("Cannot load the information about this assignment".to_string())
                }) {
                Ok(text) => text.await,
                Err(_) => "Cannot load about.txt".to_string(),
            };
            assignments.push(Assignment {
                number: i,
                about: Some(about),
            });
            i += 1;
        } else {
            break;
        }
    }

    assignments
}
