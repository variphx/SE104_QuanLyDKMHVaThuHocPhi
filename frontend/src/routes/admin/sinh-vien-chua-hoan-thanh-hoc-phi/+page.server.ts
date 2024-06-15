import type { PageServerLoad } from "./$types"

export const load: PageServerLoad = async({fetch}) => {
    const sinh_vien_chua_dong_hoc_phis = await fetch('http://localhost:8080/api/sinh-vien/chua-dong-hoc-phi/get');

    return {
        sinh_vien_chua_dong_hoc_phis: sinh_vien_chua_dong_hoc_phis.json()
    };
}