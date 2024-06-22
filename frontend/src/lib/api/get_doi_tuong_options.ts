export const get_doi_tuong_options = async () => {
  const response = await fetch(
    "http://localhost:8080/api/doi-tuong/options/get"
  );

  if (!response.ok) {
    throw new Error(await response.text());
  }

  return response.json();
};
