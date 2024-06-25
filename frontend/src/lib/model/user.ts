export type User = {
	username: string;
	hash_pwd: string;
};

export const get_user = async (username: string) => {
	const response = await fetch('http://localhost:8080/api/user/get', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(username)
	});

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<User>;
};
