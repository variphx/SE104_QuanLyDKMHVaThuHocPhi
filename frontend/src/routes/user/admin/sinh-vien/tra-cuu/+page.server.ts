import { get_sinh_vien_all } from '$lib/model/sinh_vien';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const sinh_viens = await get_sinh_vien_all();

	return {
		sinh_viens
	};
};
