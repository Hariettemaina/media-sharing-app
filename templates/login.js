document.addEventListener("DOMContentLoaded", function () {
    const loginForm = document.getElementById("login");
    loginForm.addEventListener("submit", function (event) {
        event.preventDefault();

        const userEmail = document.getElementById("Email").value;
        const password = document.getElementById("loginPassword").value;

        const mutation = `
    mutation Login($input: LoginInput!) {
        users {
            login(input: $input) {
                id
                username
            }
        }
    }
`;

        const variables = {
            input: {
                userEmail,
                password
            }
        };

        fetch('http://localhost:8000', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json',
            },
            body: JSON.stringify({
                query: mutation,
                variables,
            }),
        })
            .then(response => response.json())
            .then(data => {
                if (data.errors) {
                    console.error('GraphQL Error:', data.errors);
                    alert('Login failed. Please try again.');
                } else {
                    alert('Login successful!');
                    const user = data.data.users.login;
                    sessionStorage.setItem('userId', user.id);
                    sessionStorage.setItem('userName', user.username);
                    window.location.href = 'dashboard.html';
                }
            })
            .catch(error => {
                console.error('Network error:', error);
                alert('Network error. Please try again.');
            });
    });
});
