export const get_hoc_ky = async (id: string) => {
  const response = await fetch("http://localhost:8080/api/hoc-ky/get", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(id),
  });

  if (!response.ok) {
    throw new Error(await response.text());
  }

  return response.json();
};
