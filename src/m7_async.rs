#[cfg(test)]
mod tests {
    use std::io::Error;


    async fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
        let response = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?;

        Ok(response)
    }

    async fn my_async_call2(url: &str) -> Result<serde_json::Value, Error> {
        let response = reqwest::get(url)
        .await
        .map_err(|_| Error::new(std::io::ErrorKind::Other, "Oh no!! Some Error" ,))?;

        let json = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| Error::new(std::io::ErrorKind::Other, "Another Error"))?;

        Ok(json)
    }

    #[tokio::test]
    async fn test_calls_async_fn(){
        let url = "https://wallhaven.cc/api/v1/search";
        let res = my_async_call(url).await;
        let res2 = my_async_call2(url).await;
        match res {
            Ok(x) => println!("{:?}", x),
            Err(e) => println!("Some Error happend : {}", e)
        }

        match res2 {
            Ok(x) => println!("{:?}", x),
            Err(e) => println!("Some Error happend : {}", e)
        }
    }

}