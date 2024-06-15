export const actions = {
    lap_phieu_thu_hoc_phi: async ({ }) => {
        const request = await fetch('http://localhost:8080/api/hoc-phi/post', {
            method: "POST",
        })

        if (!request.ok) {
            // throw new Error(request.statusText);
            console.log(await request.text());
        }
    }
}