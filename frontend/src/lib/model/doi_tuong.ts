export type DoiTuong = {
	id: String;
	ten: String;
};

export const get_doi_tuong_all = async () => {
	const response = await fetch('http://localhost:8080/api/doi-tuong/chinh-sach/get/all');

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<DoiTuong[]>;
};
