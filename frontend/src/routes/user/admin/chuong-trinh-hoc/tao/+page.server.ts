import { get_current_hoc_ky } from '$lib/model/hoc_ky';
import { get_all_nganh, type Nganh } from '$lib/model/nganh';
import { get_user } from '$lib/model/user';
import { verify } from '@node-rs/argon2';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const current_hoc_ky = await get_current_hoc_ky();
	const nganhs = await get_all_nganh();

	return {
		nganhs,
		current_hoc_ky
	};
};

export const actions: Actions = {
	tao: async ({ request, fetch }) => {
		const data = await request.formData();

		const password = data.get('password')?.valueOf() as string | undefined;

		if (!password) {
			return;
		}

		const user = await get_user('admin');

		if (await verify(user.hash_pwd, password)) {
			const nganhs = await get_all_nganh();
			const current_hoc_ky = await get_current_hoc_ky();

			for (const nganh of nganhs) {
				const response = await fetch('http://localhost:8080/api/chuong-trinh-hoc/post', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({
						id_nganh: nganh.id,
						id_hoc_ky: current_hoc_ky.id
					})
				});

				if (!response.ok) {
					console.log(await response.text());
				}
			}

			return;
		}
	}
};
