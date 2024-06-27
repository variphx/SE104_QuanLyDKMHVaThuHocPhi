export type HocKy = {
	id: string;
	nam_hoc: number;
	ten: string;
};

export const get_current_hoc_ky = async () => {
	const response = await fetch('http://localhost:8080/api/params/current-hoc-ky/get');

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<HocKy>;
};
