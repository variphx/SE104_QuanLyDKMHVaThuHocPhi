export const get_que_quan = async (id: string) => {
  const response = await fetch("http://localhost:8080/api/que-quan/get", {
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
    thanh_pho: data.thanh_pho,
    tinh: data.tinh,
  };
};
