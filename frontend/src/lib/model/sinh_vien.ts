export type SinhVien = {
	id: string;
	ten: string;
	ngay_sinh: string;
	id_gioi_tinh: string;
	id_que_quan: string;
	id_doi_tuong: string;
	id_chuong_trinh_hoc: string;
};

export const get_sinh_vien_all = async () => {
	const response = await fetch('http://localhost:8080/api/sinh-vien/all/get');

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<SinhVien[]>;
};
