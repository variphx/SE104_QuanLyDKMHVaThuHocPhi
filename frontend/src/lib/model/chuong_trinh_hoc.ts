export type ChuongTrinhHoc = {
	id: string;
	id_nganh: string;
	id_hoc_ky: string;
};

export const get_chuong_trinh_hoc_all = async () => {
	const response = await fetch('http://localhost:8080/api/chuong-trinh-hoc/get/all');

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<ChuongTrinhHoc[]>;
};
