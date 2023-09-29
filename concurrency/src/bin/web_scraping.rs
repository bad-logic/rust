use std::{time::Instant, thread, sync::Arc};


fn main() -> Result<(),ureq::Error>{

    let webpages = vec![
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        ];

        let agent = ureq::AgentBuilder::new().build();
        let now = Instant::now();
        
        for web_page in &webpages {
            let web_body = agent.get(web_page).call()?.into_string()?;
        }
        println!("Time taken without threads: {:.2?}", now.elapsed());

        let now = Instant::now();
        let agent = Arc::new(agent);
        let mut handles : Vec<thread::JoinHandle<Result<(),ureq::Error>>> = Vec::new();

        for web_page in webpages{
            let agent_thread = agent.clone();
            let t = thread::spawn(move || {
                let web_body = agent_thread.get(web_page).call()?.into_string()?;
                Ok(()) 
            });
            handles.push(t);
        }

        for t in handles{
            t.join().unwrap();
        }

        println!("Time taken with threads: {:.2?}", now.elapsed());

        Ok(())

}