document.addEventListener('DOMContentLoaded', function () {
    let registrationForm = document.getElementById('registrationForm');
    registrationForm.addEventListener('submit', function (event) {

        // Email validation function
        function validateEmail(email) {
            const re = /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
            return re.test(String(email).toLowerCase());
        }

        // function isEmailVerified() {
        //     return true; 
        // }


        // Registration Form Submission
        event.preventDefault();


        let firstName = document.getElementById('firstName').value;
        let middleName = document.getElementById('middleName').value;
        let lastName = document.getElementById('lastName').value;
        let userName = document.getElementById('userName').value;
        let userEmail = document.getElementById('userEmail').value;
        let displayName = document.getElementById('displayName').value;
        let dob = document.getElementById('dob').value;
        let password = document.getElementById('password').value;
        let confirmPassword = document.getElementById('confirmPassword').value;

        // Validate email
        if (!validateEmail(userEmail)) {
            alert('Please enter a valid email address.');
            return;
        }
        // Validation
        if (password !== confirmPassword) {
            alert('Passwords do not match.');
            return;
        }
        let formData = new FormData();
        formData.append('firstName', firstName);
        formData.append('middleName', middleName);
        formData.append('lastName', lastName);
        formData.append('userName', userName);
        formData.append('userEmail', userEmail);
        formData.append('displayName', displayName);
        formData.append('dob', dob);
        formData.append('password', password);
        formData.append('confirmPassword', confirmPassword);






        var mutation = `
        mutation SignUp($input: UserInput!) {
            signup(input: $input) {
                users {
                    firstName
                    middleName
                    lastName
                    userName
                    userEmail
                    paswordHash
                    displayName
                    dateOfBirth
                }
            }
        }
        `;



        // Submit form 
        fetch('http://localhost:8000', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json',
            },
            body: JSON.stringify({
                query: mutation,
                variables: formData,
            })
        })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.json();
            })
            .then(data => {
                console.log('Signup successful:', data);

                window.location.href = 'verify_email.html';
            })
            .catch(error => {
                console.error('There was a problem with the fetch operation:', error);

            });
    });
});
