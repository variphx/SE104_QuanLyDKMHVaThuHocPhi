import type { Actions } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { get_tinh_options } from "$lib/api/get_tinh_options";
import { get_doi_tuong_options } from "$lib/api/get_doi_tuong_options";
import { get_khoa_options } from "$lib/api/get_khoa_options";

export const load: PageServerLoad = async () => {
  const tinh_options = await get_tinh_options();
  const doi_tuong_options = await get_doi_tuong_options();
  const khoa_options = await get_khoa_options();

  return {
    tinh_options: tinh_options,
    doi_tuong_options: doi_tuong_options,
    khoa_options: khoa_options,
  };
};

export const actions: Actions = {
  create_single_sinh_vien: async ({ request, fetch }) => {
    const data = await request.formData();

    const ten = data.get("ten")?.toString();
    const ngay_sinh = data.get("ngay_sinh")?.toString();
    const id_gioi_tinh = data.get("id_gioi_tinh")?.toString();
    const id_que_quan = data.get("id_que_quan")?.toString();
    const id_doi_tuong = data.get("id_doi_tuong")?.toString();
    const id_nganh = data.get("id_nganh")?.toString();

    if (
      !ten ||
      !ngay_sinh ||
      !id_gioi_tinh ||
      !id_que_quan ||
      !id_doi_tuong ||
      !id_nganh
    ) {
      throw new Error("undefined: form data");
    }

    const response = await fetch("http://localhost:8080/api/sinh-vien/post", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        ten: ten,
        ngay_sinh: ngay_sinh,
        id_gioi_tinh: id_gioi_tinh,
        id_que_quan: id_que_quan,
        id_doi_tuong: id_doi_tuong,
        id_nganh: id_nganh,
      }),
    });

    if (!response.ok) {
      throw new Error(await response.text());
    }
  },
};
