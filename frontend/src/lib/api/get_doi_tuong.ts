import { error } from "@sveltejs/kit";

export const get_doi_tuong = async (id: string) => {
  const response = await fetch("http://localhost:8080/api/doi-tuong/get", {
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
