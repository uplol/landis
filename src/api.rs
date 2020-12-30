use http::HeaderValue;
use tokio::{fs::File, io::AsyncReadExt};
use warp::Filter;

pub async fn run_api() {
    // GET /status
    let status = warp::path!("status").map(|| "OK");

    // GET /tile/{zoom}/{x}/{y}/tile.png api
    let map_tile = warp::path!("tiles" / i32 / i32 / i32 / "tile.png")
        .map(|zoom, x, y| TileRequest { zoom, x, y })
        .and_then(|req: TileRequest| async move {
            let mut file = File::open(format!("./out/{}.{}.png", req.x, req.y))
                .await
                .map_err(|_| warp::reject::not_found())?;

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .await
                .map_err(|_| warp::reject::not_found())?;

            let mut res = http::Response::new(buffer);
            res.headers_mut()
                .insert("content-type", HeaderValue::from_static("image/png"));
            Result::<_, warp::reject::Rejection>::Ok(res)
        });

    warp::serve(
        warp::get().and(
            status
                .or(map_tile)
                .or(warp::get().and(warp::fs::dir("./web"))),
        ),
    )
    .run(([0, 0, 0, 0], 3090))
    .await;
}

struct TileRequest {
    zoom: i32,
    x: i32,
    y: i32,
}
