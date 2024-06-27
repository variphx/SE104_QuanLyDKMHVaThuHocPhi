import { get_chuong_trinh_hoc_all } from '$lib/model/chuong_trinh_hoc';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const chuong_trinh_hocs = await get_chuong_trinh_hoc_all();

	return {
		chuong_trinh_hocs
	};
};
