async function logout() {
    const query = `
        mutation {
            logout
        }
    `;

    try {
        const response = await fetch('/graphql', { 
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json',
            },
            body: JSON.stringify({ query }),
        });

        const { data } = await response.json();

        console.log(data.logout); 
    } catch (error) {
        console.error('Failed to log out:', error);
    }
}
