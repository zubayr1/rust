extern crate reqwest;

fn httpget()
{
    match reqwest::get("http://youtube.local/hello")
    {
        Ok(mut response) => {
            //check if 200 ok
            if response.status() == reqwest.StatusCode::Ok
            {
                match response.text()
                {
                    Ok(text) => println!("response {}", text),
                    Err(_) => println!("response could not GET")

                }
            }
            else
            {
                println!("response status not 200")
            }
        },
        Err(_) => println!("could not make GET req")
    }
}