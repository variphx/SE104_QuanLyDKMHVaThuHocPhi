export const get_tinh_options = async () => {
  const response = await fetch("http://localhost:8080/api/que-quan/tinh/get");

  if (!response.ok) {
    throw new Error(await response.text());
  }

  return response.json();
};
