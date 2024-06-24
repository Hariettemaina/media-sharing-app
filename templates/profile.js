document.addEventListener('DOMContentLoaded', function () {
    const userId = sessionStorage.getItem('userId');

    if (!userId) {
        alert('Please login first');
        window.location.href = 'login.html';
        return;
    }

    fetchUserProfile(userId);

    const profileForm = document.getElementById('profile-form');
    profileForm.addEventListener('submit', function (event) {
        event.preventDefault();
        updateUserProfile(userId);
    });

    function fetchUserProfile(userId) {
        const query = `
            query GetUserById($input: GetUserProfile!) {
                getUserById(input: $input) {
                    id
                    name
                    email
                }
            }
        `;

        fetch('http://localhost:8000', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json',
            },
            body: JSON.stringify({
                query,
                variables: { input: { user_id: parseInt(userId) } },
            }),
        })
            .then(response => response.json())
            .then(data => {
                if (data.errors) {
                    console.error('GraphQL Error:', data.errors);
                    alert('Failed to fetch profile information.');
                } else {
                    const user = data.data.getUserById;
                    document.getElementById('name').value = user.name;
                    document.getElementById('email').value = user.email;
                }
            })
            .catch(error => {
                console.error('Network error:', error);
                alert('Network error. Please try again.');
            });
    }

    function updateUserProfile(userId) {
        const name = document.getElementById('name').value;
        const email = document.getElementById('email').value;

        const mutation = `
            mutation UpdateProfile($input: UpdateProfileInput!) {
                updateProfile(input: $input) {
                    id
                    name
                    email
                }
            }
        `;

        const variables = {
            input: {
                user_id: parseInt(userId),
                name,
                email
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
                    alert('Profile update failed. Please try again.');
                } else {
                    alert('Profile updated successfully!');
                }
            })
            .catch(error => {
                console.error('Network error:', error);
                alert('Network error. Please try again.');
            });
    }
});
