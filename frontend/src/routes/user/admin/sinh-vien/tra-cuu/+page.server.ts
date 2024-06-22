import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch }) => {
  const response = await fetch("http://localhost:8080/api/sinh-vien/all/get");

  if (!response.ok) {
    throw new Error(await response.text());
  }

  return {
    sinh_viens: response.json(),
  };
};
