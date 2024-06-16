export const get_khoa = async (id: string) => {
  const response = await fetch("http://localhost:8080/api/chuong-trinh-hoc/khoa/get", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(id),
  });

  if (!response.ok) {
    throw new Error(await response.text());
  }

  const data = await response.json();

  return {
    ten: data.ten,
  };
};
