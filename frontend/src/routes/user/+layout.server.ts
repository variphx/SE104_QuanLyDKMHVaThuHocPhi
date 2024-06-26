import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const session_id = cookies.get('session_id');

	if (!session_id) {
		redirect(302, '/login');
	}

	return {
		session_id
	};
};
