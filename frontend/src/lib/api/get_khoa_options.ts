export const get_khoa_options = async () => {
  const response = await fetch(
    "http:/localhost:8080/api/chuong-trinh-hoc/khoa/options/get"
  );

  if (!response.ok) {
    throw new Error(await response.text());
  }

  return response.json();
};
