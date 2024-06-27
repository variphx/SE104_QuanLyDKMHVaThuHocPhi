export type Khoa = {
	id: string;
	ten: string;
};

export const get_all_khoa = async () => {
	const response = await fetch('http://localhost:8080/api/chuong-trinh-hoc/khoa/all/get');

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<Khoa[]>;
};
