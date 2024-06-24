import { get_khoa_options } from "$lib/api/get_khoa_options";
import { get_nganh_options } from "$lib/api/get_nganh_options";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
  const khoa_options = await get_khoa_options();
  let nganhs = [];

   for (const khoa_option of khoa_options) {
    const nganhs_of_khoa = await get_nganh_options(khoa_option.id);
    nganhs.push(nganhs_of_khoa);
  }

  return {
    nganhs,
  };
};
