import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
    const hoc_phis = await fetch('http://localhost:8080/api/hoc-phi/get', {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            id_sinh_vien: params.id
        })
    }).then((response) => response.json());

    return {
        hoc_phis: hoc_phis,
        id_sinh_vien: params.id,
    };
}