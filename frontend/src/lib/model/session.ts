export type Session = {
	id: string;
	username: string;
};

export const get_session = async (id: string) => {
	const response = await fetch('http://localhost:8080/api/session/get', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(id)
	});

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<Session>;
};
