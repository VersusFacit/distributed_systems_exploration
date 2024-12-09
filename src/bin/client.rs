use reqwest::Client;



#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let get_count = if let Some(arg) = std::env::args().nth(1) {
        arg.parse::<usize>().expect("Please provide a number for the count of get requests to kick off.")
    } else {
        3
    };
    let client = Client::new();
    let simple_request = client.get("http://127.0.0.1:8085/hey");

    let handles = (0..get_count).map(|_| {
        tokio::spawn(simple_request.try_clone().expect("How would clone fail?").send())
    }).collect::<Vec<tokio::task::JoinHandle<_>>>();
    let mut outputs = Vec::with_capacity(handles.len());
    for handle in handles {
        outputs.push(handle.await.expect("client thread failed"))
    }
    println!("{:?}", outputs);
    for output in outputs {
        match output {
            Ok(x) => println!("{}", x.text().await?),
            Err(_) => panic!("COULDNT GET TEXT"),
        }
    }
    Ok(())
}
