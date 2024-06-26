export type Nganh = {
	id: string;
	ten: string;
};

export const get_nganh = async (id: string) => {
	const response = await fetch('http://localhost:8080/api/chuong-trinh-hoc/nganh/get', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(id)
	});

	if (response.status === 404) {
		return null;
	}

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<Nganh>;
};

export const get_all_nganh = async () => {
	const response = await fetch('http://localhost:8080/api/chuong-trinh-hoc/nganh/all/get');

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<Nganh[]>;
};
