# simkl

Data structures to parse [SIMKL](https://simkl.com/) API.

The goal of the lib is to help you creating requests on the SIMKL API and parsing the JSON results using **serde**.
You are free to use any library to perform your HTTP request.

## Usage

```rust
// Prepare your request
let request = SearchRequest::new("Breaking Bad")
    .with_type(MediaType::Show)
    .with_limit(10);

// The lib is providing the URL and related headers
let url = request.build_url();
let headers = request.headers();

// Make your HTTP call using any lib (reqwest, ureq, ...)
let response = reqwest::get(&url).await?.text().await?;

// Parse the response with this lib
let search_results: SearchResponse = serde_json::from_str(&response)?;
```

## Main endpoints

* `GET /search/{type}`: media search
* `GET /movies/{id}`: movie information
* `GET /tv/{id}`: TV information
* `GET /anime/{id}`: anime information
* `GET /tv/{id}/episodes/{season}`: season episode
* `GET /sync/all-items/{type}`:
* `POST /sync/add-to-list`:
* `POST /sync/remove-from-list`:
* `GET /sync/activities`: user activity
* `GET /users/settings`: user settings
