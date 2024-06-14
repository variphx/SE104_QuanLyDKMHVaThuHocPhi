import type { PageServerLoad } from './$types';


export const load: PageServerLoad = async ({ params }) => {
    const request = await fetch("http://localhost:8080/api/mon-hoc-mo/get",
        {
            headers: {
                "Content-Type": "application/json"
            },
            method: "POST",
            body: JSON.stringify({
                id_sinh_vien: params.id
            })
        }
    );

    return {
        id_sinh_vien: params.id,
        mon_hocs: request.json(),
    };
}

export const actions = {
    default: async () => { },
}