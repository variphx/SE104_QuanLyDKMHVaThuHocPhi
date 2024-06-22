export const get_thanh_pho_options = async (id: string) => {
  const response = await fetch(
    "http://localhost:8080/api/que-quan/thanh-pho/get",
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

  return response.json();
};
