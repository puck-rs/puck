use puck::{body::Body, Response};

/// Returns the index page to the client.
///
/// You can mount this anywhere, but make sure that you mount an instance of `js` at
/// `/<path of this route>/js`.
pub fn index() -> Response {
    Response::build()
        .header("Content-Type", "text/html")
        .body(Body::from_string(
            r#"
        <!DOCTYPE html>
        <html>
            <head>
                <script src="./js"></script>
            </head>
            <body>
            </body>
        </html>
        "#,
        ))
        .build()
}

/// Returns the JS needed for the application to the client.
///
/// You need to mount this at <wherever you mounted the HTML file>/js
pub fn js() -> Response {
    Response::build()
        .header("Content-Type", "application/javascript")
        .body(Body::from_string(include_str!("../client/index.js")))
        .build()
}
