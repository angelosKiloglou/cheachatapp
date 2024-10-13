export async function fetchUserId(): Promise<number> {
    const currentUserUrl = 'http://localhost:9000/get-current-user';
    const res = await fetch(currentUserUrl, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include'

    });

    if (res.ok) {
        return await res.json();
    } else {
        return -1;
    }
}