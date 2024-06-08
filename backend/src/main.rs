use axum::Router;
use context::Context;
use tower_http::cors::CorsLayer;

const PORT: u16 = 8080;

mod context;
mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // khởi tạo biến môi trường
    dotenv::dotenv()?;

    // kết nối với database
    let db_pool = {
        let url = std::env::var("DATABASE_URL")?;
        sqlx::PgPool::connect(&url).await?
    };

    // chạy migrator
    sqlx::migrate!().run(&db_pool).await?;

    // khởi tạo listener
    let listener = {
        let address = std::net::SocketAddr::from(([0, 0, 0, 0], PORT));
        tokio::net::TcpListener::bind(address).await?
    };

    let context = Context::new(db_pool);

    // tạo router cho api
    let api = Router::new()
        .nest("/chuong-trinh-hoc", handler::chuong_trinh_hoc::router())
        .nest("/doi-tuong", handler::doi_tuong::router())
        .nest("/hoc-ky", handler::hoc_ky::router())
        .nest("/hoc-phi", handler::hoc_phi::router())
        .nest("/mon-hoc", handler::mon_hoc::router())
        .nest("/mon-hoc-mo", handler::mon_hoc_mo::router())
        .route("/nganh", handler::nganh::method_router())
        // .route("/que-quan", handler::que_quan::method_router())
        .route("/sinh-vien", handler::sinh_vien::method_router())
        .route("/user", handler::user::method_router())
        .nest(
            "/user",
            Router::new().route("/session", handler::user::session::method_router()),
        );

    // nest api vào app
    let app = Router::new()
        .nest("/api", api)
        .layer(CorsLayer::permissive())
        .with_state(context);

    // chạy app
    axum::serve(listener, app).await?;

    Ok(())
}
