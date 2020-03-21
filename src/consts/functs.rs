pub async fn body_to_string(body:hyper::Body) -> core::result::Result<String, Box<dyn std::error::Error>> {
    let mut string: String = String::from_utf8(
    hyper::body::to_bytes(body).await
    .unwrap()
    .iter().cloned().collect::<Vec<u8>>())
    .expect("Converting bytes to string failed");
    Ok(string)
}