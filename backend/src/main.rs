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

    // tạo router cho api
    let api = Router::new()
        .route(
            "/chuong-trinh-hoc",
            handler::chuong_trinh_hoc::method_router(),
        )
        .route("/doi-tuong", handler::doi_tuong::method_router())
        .nest(
            "/doi-tuong",
            Router::new()
                .route(
                    "/chinh-sach",
                    handler::doi_tuong::chinh_sach::method_router(),
                )
                .route("/vung-mien", handler::doi_tuong::vung_mien::method_router()),
        )
        .route("/hoc-ky", handler::hoc_ky::method_router())
        .route("/hoc-phi", handler::hoc_phi::method_router())
        .route("/khoa", handler::khoa::method_router())
        .route("/mon-hoc", handler::mon_hoc::method_router())
        .nest(
            "/mon-hoc",
            Router::new().route("/mo", handler::mon_hoc::mo::method_router()),
        )
        .route("/nganh", handler::nganh::method_router())
        .route("/que-quan", handler::que_quan::method_router())
        .route("/sinh-vien", handler::sinh_vien::method_router())
        .route("/user", handler::user::method_router());

    let state = Context::new(db_pool);

    // nest api vào app
    let app = Router::new()
        .nest("/api", api)
        .layer(CorsLayer::permissive())
        .with_state(state);

    // chạy app
    axum::serve(listener, app).await?;

    Ok(())
}
