import { get_all_khoa } from '$lib/model/khoa';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const khoas = await get_all_khoa();

	return {
		khoas
	};
};

export const actions: Actions = {
	tao: async ({ request, fetch }) => {
		const data = await request.formData();

		const id = data.get('id')?.valueOf() as string | undefined;
		const ten = data.get('ten')?.valueOf() as string | undefined;
		const id_khoa = data.get('id_khoa')?.valueOf() as string | undefined;
		const loai = data.get('loai')?.valueOf() as string | undefined;
		const so_tiet = data.get('so_tiet')?.valueOf() as string | undefined;

		if (!(id && ten && id_khoa && loai && so_tiet)) {
			throw new Error();
		}

		const body = {
			id,
			ten,
			id_khoa,
			loai,
			so_tiet
		};

		const response = await fetch('http://localhost:8080/api/mon-hoc/post', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});

		if (!response.ok) {
			throw new Error(await response.text());
		}
	}
};
