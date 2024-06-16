export const get_chuong_trinh_hoc = async (id: string) => {
  const response = await fetch(
    "http://localhost:8080/api/chuong-trinh-hoc/get",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(id),
    }
  );

  if (!response.ok) {
    throw new Error(await response.text());
  }

  const data = await response.json();

  return {
    id_nganh: data.id_nganh,
    id_hoc_ky: data.id_hoc_ky,
  };
};
