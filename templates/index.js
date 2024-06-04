document.addEventListener('DOMContentLoaded', function () {
    let registrationForm = document.getElementById('registrationForm');
    registrationForm.addEventListener('submit', function (event) {
        event.preventDefault(); // Prevent the default form submission
        const socket = new WebSocket('ws://localhost:8000');

        socket.addEventListener('open',(event)=>{
            console.log('Connected', event);
        });

        socket.addEventListener('message', (event)=> {
            console.log('Message form server: ', event.data);
        });


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








// document.addEventListener('DOMContentLoaded', function () {
//     let registrationForm = document.getElementById('registrationForm');
//     registrationForm.addEventListener('submit', function (event) {
//         event.preventDefault(); // Prevent the default form submission

//         // Establish WebSocket connection
//         let socket = new WebSocket('ws://localhost:8000');

//         socket.onopen((event) => {
//             let message= JSON.parse(event.data);
//             console.log('Connected 1', event);
//         })

//         // Connection opened
//         // socket.addEventListener('open', (event) => {
//         //     console.log('Connected', event);
//         // });

//         // Listen for messages from the server
//         socket.addEventListener('message', (event) => {
//             console.log('Message from server: ', event.data);

//             // Assuming the server sends a message indicating success
//             if (event.data === 'signup_success') {
//                 alert('Signup successful. Please check your email to verify your account.');
//                 if (verificationCode) {
//                     window.location.href = 'verify_email.html';
//                 }
//             }
//         });

//         // Close event listener
//         socket.addEventListener('close', (event) => {
//             console.log('Socket closed', event);
//             socket = null; // Clear the socket reference
//         });

//         // Function to safely send messages
//         function safeSend(message) {
//             if (socket.readyState === WebSocket.OPEN) {
//                 socket.send(JSON.stringify(message));
//             } else {
//                 console.warn('WebSocket is not open. Message not sent.');
//             }
//         }

//         // Extract form data
//         let firstName = document.getElementById('firstName').value;
//         let middleName = document.getElementById('middleName').value;
//         let lastName = document.getElementById('lastName').value;
//         let userName = document.getElementById('userName').value;
//         let userEmail = document.getElementById('userEmail').value;
//         let displayName = document.getElementById('displayName').value;
//         let dob = document.getElementById('dob').value;
//         let password = document.getElementById('password').value;
//         let confirmPassword = document.getElementById('confirmPassword').value;

//         // Validate password match
//         if (password !== confirmPassword) {
//             alert('Passwords do not match.');
//             return;
//         }

//         // Prepare the GraphQL mutation
//         let mutation = `
//         mutation SignUp($input: UserInput!) {
//             users{
//                 signup(input: $input){
//                 firstName
//                 middleName
//                 lastName
//                 username
//                 userEmail
//                 passwordHash
//                 displayName
//                 dateOfBirth
//             }
//             }
//         }
//         `;

//         let variables = {
//             input: {
//                 firstName,
//                 middleName,
//                 lastName,
//                 username: userName,
//                 userEmail,
//                 displayName,
//                 passwordHash: password,
//                 dateOfBirth: dob,
//             }
//         };

//         // Send the request to the GraphQL server
//         fetch('http://localhost:8000', {
//             method: 'POST',
//             headers: {
//                 'Content-Type': 'application/json',
//                 'Accept': 'application/json',
//             },
//             body: JSON.stringify({
//                 query: mutation,
//                 variables,
//             }),
//         })
//             .then(response => response.json())
//             .then(data => {
//                 if (data.errors) {
//                     console.error('GraphQL Error:', data.errors);
//                     alert('Signup failed. Please try again.');
//                 } else {
//                     // Notify the server about the successful signup
//                     safeSend({ type: 'signup_notification' });
//                 }
//             })
//             .catch(error => {
//                 console.error('Network error:', error);
//                 alert('Network error. Please try again.');
//             });
//     });
// });
