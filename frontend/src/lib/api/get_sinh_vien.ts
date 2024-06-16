import { error } from "@sveltejs/kit";

export const get_sinh_vien = async (id: string) => {
  const response = await fetch("http://localhost:8080/api/sinh-vien/get", {
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

  return data;
};
