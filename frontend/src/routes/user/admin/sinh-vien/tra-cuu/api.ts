import type { State } from "@vincjo/datatables/remote";

export const reload = async (state: State) => {
  const response = await fetch(
    `https://jsonplaceholder.typicode.com/todos?${getParams(state)}`
  );
  return response.json();
};

const getParams = (state: State) => {
  const { pageNumber, rowsPerPage, sort, filters, search } = state;

  let params = {
    page_number: pageNumber,
    rows_per_page: rowsPerPage,
    sort: sort,
    filters: filters,
    search: search,
  };

  return params;
};
