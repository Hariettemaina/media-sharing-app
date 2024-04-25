#[cfg(test)]
mod tests {
    use async_graphql::{EmptySubscription, Schema};
    use diesel_async::pooled_connection::deadpool::Pool;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use reqwest::multipart;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tokio::fs;

    use crate::graphql_schema::images::mutation::upload::UploadMedia;
    //use crate::schema;

    #[tokio::test]
    async fn test_upload_file() {

        

        // Create a temporary file to simulate an uploaded file
        let temp_file_path = "./uploads/test_upload.jpg";
        // create a dir
        let dir = Path::new(&temp_file_path)
            .parent()
            .expect("Failed to get parent directory");
        fs::create_dir_all(&dir)
            .await
            .expect("Failed to create directory");

        let mut file = File::create(temp_file_path).expect("Failed to create temporary file");
        file.write_all(b"test image content")
            .expect("Failed to write to temporary file");

        // Read the file content into a Vec<u8>
        let file_content = fs::read(temp_file_path)
            .await
            .expect("Failed to read file content");

        // Create a multipart request with the temporary file
        let client = reqwest::Client::new();
        let form = multipart::Form::new()
        .text("operations", r#"{ "query": "mutation ($file: Upload!) { upload(file: $file) }", "variables": { "file": null } }"#)
        .text("map", r#"{ "0": ["variables.file"] }"#)
        .part("0", multipart::Part::bytes(file_content)
            .file_name("test_upload.jpg") // Set the file name
            .mime_str("image/jpeg").unwrap()); // Set the MIME type

        // Send the request to  GraphQL server
        let response = client
            .post("http://localhost:8000")
            .multipart(form)
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success());

        // let result = schema.execute().await;

        // Cleanup
        //std::fs::remove_file(temp_file_path).expect("Failed to remove temporary file");
    }
}
// create a temporary file: It  then creates a temporary file with some content to simulate an uploaded file.
// create a multipart request: It constructs a multipart request that includes the temporary file.
//the request is structured according to the GraphQL multipart request specification,
//which requires specifying the GraphQL query and variables in a JSON string, along with a map that associates the file part with the variable in the query.
// Sends the Request: It sends the request to the GraphQL server using reqwest.
// asserts the result: It checks that the response status indicates success, indicating that the file upload was processed successfully.
// cleans Up: It removes the temporary file after the test.
//we use reqwest, as it  supports multipart requests.
