use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct HocPhi {
    nam_hoc: i32,
    ten_hoc_ky: String,
    tong: f64,
    da_thanh_toan: f64,
}

#[derive(Deserialize)]
struct HocPhiQueryPayload {
    id_sinh_vien: String,
    // id_hoc_ky: String,
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
) -> Result<Json<Vec<HocPhi>>, StatusCode> {
    let hoc_phis = sqlx::query_as::<_, HocPhi>(
        "SELECT HOC_KY.nam_hoc, HOC_KY.ten as ten_hoc_ky, HOC_PHI.tong, HOC_PHI.da_thanh_toan FROM HOC_PHI, HOC_KY
            WHERE id_sinh_vien = $1
                AND id_hoc_ky = HOC_KY.id",
    )
    .bind(payload.id_sinh_vien)
    // .bind(payload.id_hoc_ky)
    .fetch_all(context.pool())
    .await
    .unwrap();

    Ok(Json(hoc_phis))
}

async fn post(State(context): State<Context>) -> impl IntoResponse {
    let gia_tin_chi_lt = match sqlx::query_scalar::<_, i64>(
        "SELECT gia_tin_chi_lt FROM THAM_SO
            WHERE id = 1",
    )
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
        }
    };

    let gia_tin_chi_th = match sqlx::query_scalar::<_, i64>(
        "SELECT gia_tin_chi_th FROM THAM_SO
            WHERE id = 1",
    )
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
        }
    };

    let id_hoc_ky = match sqlx::query_scalar::<_, String>(
        "SELECT id_hoc_ky FROM THAM_SO
                WHERE id = 1",
    )
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
        }
    };

    let id_sinh_viens = match sqlx::query_scalar::<_, String>(
        "SELECT SINH_VIEN.id FROM SINH_VIEN, DANG_KY_MON_HOC
                WHERE SINH_VIEN.id = DANG_KY_MON_HOC.id_sinh_vien
                    AND DANG_KY_MON_HOC.id_hoc_ky = $1",
    )
    .bind(id_hoc_ky.as_str())
    .fetch_all(context.pool())
    .await
    {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
        }
    };

    for id_sinh_vien in id_sinh_viens {
        let he_so_hoc_phi = match sqlx::query_scalar::<_, f64>(
            "SELECT he_so_hoc_phi FROM DOI_TUONG, SINH_VIEN
                    WHERE DOI_TUONG.id = SINH_VIEN.id_doi_tuong
                        AND SINH_VIEN.id = $1",
        )
        .bind(&id_sinh_vien)
        .fetch_one(context.pool())
        .await
        {
            Ok(value) => value,
            Err(error) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        };

        // let id_mon_hocs = match sqlx::query_scalar::<_, String>(
        //     "SELECT id_mon_hoc FROM CHI_TIET_DANG_KY_MON_HOC, DANG_KY_MON_HOC
        //             WHERE DANG_KY_MON_HOC.id_sinh_vien = $1
        //                 AND DANG_KY_MON_HOC.id_hoc_ky = $2
        //                 AND CHI_TIET_DANG_KY_MON_HOC.id_dang_ky_mon_hoc = DANG_KY_MON_HOC.id",
        // )
        // .bind(&id_sinh_vien)
        // .bind(&id_hoc_ky)
        // .fetch_all(context.pool())
        // .await
        // {
        //     Ok(value) => value,
        //     Err(error) => {
        //         return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
        //     }
        // };

        let tin_chi_lt_count = match sqlx::query_scalar::<_, i64>(
            "SELECT SUM (MON_HOC.so_tin_chi) FROM MON_HOC, CHI_TIET_DANG_KY_MON_HOC, DANG_KY_MON_HOC
                WHERE DANG_KY_MON_HOC.id_sinh_vien = $1
                    AND DANG_KY_MON_HOC.id_hoc_ky = $2
                    AND CHI_TIET_DANG_KY_MON_HOC.id_dang_ky_mon_hoc = DANG_KY_MON_HOC.id
                    AND CHI_TIET_DANG_KY_MON_HOC.id_mon_hoc = MON_HOC.id
                    AND MON_HOC.loai = 'LT'",
        )
        .bind(&id_sinh_vien)
        .bind(&id_hoc_ky)
        .fetch_one(context.pool())
        .await
        {
            Ok(value) => value,
            Err(_) => 
                0
            
        };

        let tin_chi_th_count = match sqlx::query_scalar::<_, i64>(
            "SELECT SUM (so_tin_chi) FROM MON_HOC, CHI_TIET_DANG_KY_MON_HOC, DANG_KY_MON_HOC
                WHERE DANG_KY_MON_HOC.id_sinh_vien = $1
                    AND DANG_KY_MON_HOC.id_hoc_ky = $2
                    AND CHI_TIET_DANG_KY_MON_HOC.id_dang_ky_mon_hoc = DANG_KY_MON_HOC.id
                    AND CHI_TIET_DANG_KY_MON_HOC.id_mon_hoc = MON_HOC.id
                    AND MON_HOC.loai = 'TH'",
        )
        .bind(&id_sinh_vien)
        .bind(&id_hoc_ky)
        .fetch_one(context.pool())
        .await
        {
            Ok(value) => value,
            Err(_) => {
                0
            }
        };

        // println!("{:?}{:?}", tin_chi_lt_count, tin_chi_th_count);

        let tong = (gia_tin_chi_lt * tin_chi_lt_count + gia_tin_chi_th * tin_chi_th_count) as f64
            * he_so_hoc_phi;

        match sqlx::query(
            "INSERT INTO HOC_PHI (id_sinh_vien, id_hoc_ky, tong, da_thanh_toan)
            VALUES (
                $1,
                $2,
                $3,
                0
            )",
        )
        .bind(&id_sinh_vien)
        .bind(&id_hoc_ky)
        .bind(tong)
        .execute(context.pool())
        .await
        {
            Ok(_) => (),
            Err(error) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        };
    }

    (StatusCode::CREATED).into_response()
}
