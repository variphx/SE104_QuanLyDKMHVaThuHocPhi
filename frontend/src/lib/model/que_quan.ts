export type QueQuan = {
	id: string;
	thanh_pho: string;
	tinh: string;
};

export const get_que_quan_all = async () => {
	const response = await fetch('http://localhost:8080/api/que-quan/get/all');

	if (!response.ok) {
		throw new Error(await response.text());
	}

	return response.json() as Promise<QueQuan[]>;
};
