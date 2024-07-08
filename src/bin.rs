fn main() {}

#[cfg(test)]
mod tests {

    use futures::TryStreamExt;
    use globescraper::client;
    use std::collections::HashMap;

    #[tokio::test]
    async fn get_not_enough_information_result_test() {
        let globe_client = client::GlobeScraperClient::<String>::new(
            String::from("bat ctle"),
            String::from("description"),
        );

        let mut props = Box::new(HashMap::<String, String>::new());
        match globe_client {
            Ok(mut client) => {
                let mut stream = client.get_page(&mut props);

                while let Ok(Some(_)) = stream.try_next().await {}
            }
            Err(_) => todo!(),
        }

        let result: String = props.get("description").unwrap().clone();

        assert!(result.contains("I do not have enough information"));
    }

    #[tokio::test]
    async fn get_results() {
        let globe_client = client::GlobeScraperClient::<String>::new(
            String::from("berat castle"),
            String::from("description"),
        );

        let mut props = Box::new(HashMap::<String, String>::new());
        match globe_client {
            Ok(mut client) => {
                let mut stream = client.get_page(&mut props);

                while let Ok(Some(_)) = stream.try_next().await {}
            }
            Err(_) => todo!(),
        }

        let result: String = props.get("description").unwrap().clone();

        assert_eq!(result.contains("I do not have enough information"), false);
    }
}
