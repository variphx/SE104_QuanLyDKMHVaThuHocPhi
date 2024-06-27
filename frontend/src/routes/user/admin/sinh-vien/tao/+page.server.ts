import { get_chuong_trinh_hoc_all } from '$lib/model/chuong_trinh_hoc';
import { get_doi_tuong_all } from '$lib/model/doi_tuong';
import { get_que_quan_all } from '$lib/model/que_quan';
import { hash } from '@node-rs/argon2';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const chuong_trinh_hocs = await get_chuong_trinh_hoc_all();
	const que_quans = await get_que_quan_all();
	const doi_tuongs = await get_doi_tuong_all();

	return {
		chuong_trinh_hocs,
		que_quans,
		doi_tuongs
	};
};

export const actions: Actions = {
	tao: async ({ request, fetch }) => {
		const data = await request.formData();

		const ten = data.get('ten')?.toString();
		const ngay_sinh = data.get('ngay_sinh')?.toString();
		const id_gioi_tinh = data.get('id_gioi_tinh')?.toString();
		const id_que_quan = data.get('id_que_quan')?.toString();
		const id_doi_tuong = data.get('id_doi_tuong')?.toString();
		const id_chuong_trinh_hoc = data.get('id_chuong_trinh_hoc')?.toString();

		const response = await fetch('http://localhost:8080/api/sinh-vien/post', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				ten,
				ngay_sinh,
				id_gioi_tinh,
				id_que_quan,
				id_doi_tuong,
				id_chuong_trinh_hoc
			})
		});

		if (!response.ok) {
			throw new Error(await response.text());
		}

		const username = await response.json();
		const hash_pwd = await hash(username);

		const user_response = await fetch('http://localhost:8080/api/user/post', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username,
				hash_pwd
			})
		});

		if (!user_response.ok) {
			throw new Error(await user_response.text());
		}
	}
};
