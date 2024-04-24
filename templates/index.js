document.addEventListener('DOMContentLoaded', function () {
    let registrationForm = document.getElementById('registrationForm');
    registrationForm.addEventListener('submit', function (event) {
        event.preventDefault(); // Prevent the default form submission
        const urlParams = new URLSearchParams(window.location.search);
        const verificationCode = urlParams.get('code');
        // Extract form data
        let firstName = document.getElementById('firstName').value;
        let middleName = document.getElementById('middleName').value;
        let lastName = document.getElementById('lastName').value;
        let userName = document.getElementById('userName').value;
        let userEmail = document.getElementById('userEmail').value;
        let displayName = document.getElementById('displayName').value;
        let dob = document.getElementById('dob').value;
        console.log(dob)
        let password = document.getElementById('password').value;
        let confirmPassword = document.getElementById('confirmPassword').value;

        // Validate password match
        if (password !== confirmPassword) {
            alert('Passwords do not match.');
            return;
        }

        // Prepare the GraphQL mutation
        let mutation = `
        mutation SignUp($input: UserInput!) {
            users{
                signup(input: $input){
                firstName
                middleName
                lastName
                username
                userEmail
                passwordHash
                displayName
                dateOfBirth
            }
            }
            }
    `;

        
        let variables = {
            input: {
                firstName,
                middleName,
                lastName,
                username: userName,
                userEmail,
                displayName,
                passwordHash: password,
                dateOfBirth: dob,
            }
        };

        // Send the request to the GraphQL server
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
                    alert('Signup failed. Please try again.');
                } else {
                    console.log('Signup successful:', data.data.users.signup);
                    alert('Signup successful! Please check your email to verify your account.');
                }
                if (verificationCode) {
                    window.location.href = 'verify_email.html'
                }
            })
            .catch(error => {
                console.error('Network error:', error);
                alert('Network error. Please try again.');
            });
    });
});
