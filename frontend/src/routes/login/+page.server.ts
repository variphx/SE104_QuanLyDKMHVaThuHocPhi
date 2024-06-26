import { get_user } from '$lib/model/user';
import { redirect, type Cookies } from '@sveltejs/kit';
import type { Actions } from './$types';

import { verify } from '@node-rs/argon2';
import type { Session } from '$lib/model/session';

export const actions: Actions = {
	dang_nhap: async ({ request, cookies }) => {
		const form_data = await request.formData();

		const username = form_data.get('username')?.toString();
		const password = form_data.get('password')?.toString();

		if (!username) {
			throw new Error();
		}

		if (!password) {
			throw new Error();
		}

		const user = await get_user(username);
		console.log(user.hash_pwd);

		if (await verify(user.hash_pwd, password)) {
			await set_cookies_session_id(cookies, username);
			redirect(302, `/user/${username}`);
		}
	}
};

const set_cookies_session_id = async (cookies: Cookies, username: string) => {
	const session_id = crypto.randomUUID();

	cookies.set('session_id', session_id, {
		path: '/',
		secure: true,
		httpOnly: true
	});

	const session: Session = {
		id: session_id,
		username
	};

	await post_session(session);
};

const post_session = async (session: Session) => {
	const response = await fetch('http://localhost:8080/api/session/post', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(session)
	});

	if (!response.ok) {
		throw new Error(await response.text());
	}
};
