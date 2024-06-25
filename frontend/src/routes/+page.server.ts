import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { get_session } from '$lib/model/session';

export const load: PageServerLoad = async ({ cookies }) => {
	const session_id = cookies.get('session_id');

	if (!session_id) {
		redirect(302, '/login');
	}

	const session = await get_session(session_id);
	redirect(302, `/user/${session.username}`);
};
