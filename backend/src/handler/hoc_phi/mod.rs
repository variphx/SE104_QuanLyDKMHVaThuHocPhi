use axum::{extract::State, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct HocPhi {
    id_sinh_vien: String,
    id_hoc_ky: String,
    tong: i64,
    da_thanh_toan: i64,
}

#[derive(Deserialize)]
struct HocPhiQueryPayload {
    id_sinh_vien: String,
    id_hoc_ky: String,
}

#[derive(Deserialize)]
struct HocPhiCreatePayload {
    id_sinh_vien: String,
    id_hoc_ky: String,
}

pub fn router() -> Router<Context> {
    // axum::routing::get(get).post(post)
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<HocPhiQueryPayload>,
) -> Json<HocPhi> {
    let hoc_phi = sqlx::query_as::<_, HocPhi>(
        "SELECT * FROM HOC_PHI
            WHERE id_sinh_vien = $1
                AND id_hoc_ky = $2",
    )
    .bind(payload.id_sinh_vien)
    .bind(payload.id_hoc_ky)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Json(hoc_phi)
}

async fn post(State(context): State<Context>, Json(payload): Json<HocPhiCreatePayload>) {
    let gia_tin_chi_lt = sqlx::query_scalar::<_, i64>(
        "SELECT gia_tin_chi_lt FROM THAM_SO
            WHERE id = 1",
    )
    .fetch_one(context.pool())
    .await
    .unwrap();

    let gia_tin_chi_th = sqlx::query_scalar::<_, i64>(
        "SELECT gia_tin_chi_th FROM THAM_SO
            WHERE id = 1",
    )
    .fetch_one(context.pool())
    .await
    .unwrap();

    let mut tin_chi_lt_count = 0;
    let mut tin_chi_th_count = 0;

    let he_so_mien_giam = sqlx::query_scalar::<_, f64>(
        "SELECT mien_giam FROM DOI_TUONG, SINH_VIEN
            WHERE DOI_TUONG.id = SINH_VIEN.id_doi_tuong
                AND SINH_VIEN.id = $1",
    )
    .bind(&payload.id_sinh_vien)
    .fetch_one(context.pool())
    .await
    .unwrap();

    let id_mon_hoc_mos = sqlx::query_scalar::<_, String>(
        "SELECT id_mon_hoc_mo FROM CHI_TIET_DANG_KY_MON_HOC, DANG_KY_MON_HOC
            WHERE DANG_KY_MON_HOC.id_sinh_vien = $1
                AND DANG_KY_MON_HOC.id_hoc_ky = $2
                AND CHI_TIET_DANG_KY_MON_HOC.id_dang_ky_mon_hoc = DANG_KY_MON_HOC.id",
    )
    .bind(&payload.id_sinh_vien)
    .bind(&payload.id_hoc_ky)
    .fetch_all(context.pool())
    .await
    .unwrap();

    for id_mon_hoc_mo in id_mon_hoc_mos {
        let (loai, so_tin_chi) = sqlx::query_scalar::<_, (String, i64)>(
            "SELECT loai, so_tin_chi FROM MON_HOC_MO, MON_HOC
                WHERE MON_HOC_MO.id_mon_hoc = MON_HOC.id
                    AND MON_HOC_MO.id = $1",
        )
        .bind(id_mon_hoc_mo)
        .fetch_one(context.pool())
        .await
        .unwrap();

        match loai.as_str() {
            "LT" => tin_chi_lt_count += so_tin_chi,
            "TH" => tin_chi_th_count += so_tin_chi,
            _ => unreachable!(),
        }
    }

    let tong = (gia_tin_chi_lt * tin_chi_lt_count + gia_tin_chi_th * tin_chi_th_count) as f64
        * he_so_mien_giam;

    sqlx::query(
        "INSERT INTO HOC_PHI (id_sinh_vien, id_hoc_ky, tong, da_thanh_toan)
            VALUES (
                $1,
                $2,
                $3,
                0
            )",
    )
    .bind(payload.id_sinh_vien)
    .bind(payload.id_hoc_ky)
    .bind(tong)
    .execute(context.pool())
    .await
    .unwrap();
}
