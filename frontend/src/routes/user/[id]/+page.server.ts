import type { PageServerLoad } from "./$types";
import { get_sinh_vien } from "$lib/api/get_sinh_vien";
import { get_que_quan } from "$lib/api/get_que_quan";
import { get_doi_tuong } from "$lib/api/get_doi_tuong";
import { get_chuong_trinh_hoc } from "$lib/api/get_chuong_trinh_hoc";
import { get_nganh } from "$lib/api/get_nganh";
import { get_khoa } from "$lib/api/get_khoa";
import { get_hoc_ky } from "$lib/api/get_hoc_ky";

export const load: PageServerLoad = async ({ params }) => {
  const id_sinh_vien = params.id;

  const sinh_vien = await get_sinh_vien(id_sinh_vien);

  const que_quan = await get_que_quan(sinh_vien.id_que_quan);
  const doi_tuong = await get_doi_tuong(sinh_vien.id_doi_tuong);
  const chuong_trinh_hoc = await get_chuong_trinh_hoc(
    sinh_vien.id_chuong_trinh_hoc
  );
  const nganh = await get_nganh(chuong_trinh_hoc.id_nganh);
  const hoc_ky = await get_hoc_ky(chuong_trinh_hoc.id_hoc_ky);
  const khoa = await get_khoa(nganh.id_khoa);

  return {
    sinh_vien: {
      id: id_sinh_vien,
      ten: sinh_vien.ten,
      que_quan: que_quan,
      doi_tuong: doi_tuong.ten,
      nganh: nganh.ten,
      khoa: khoa.ten,
      hoc_ky_nhap_hoc: hoc_ky,
    },
  };
};
