document.addEventListener('DOMContentLoaded', function () {
    const verifyButton = document.querySelector('.cta-button');
    const verificationStatus = document.getElementById('verificationStatus');

    verifyButton.addEventListener('click', function (event) {
        event.preventDefault();


        const urlParams = new URLSearchParams(window.location.search);
        const verificationCode = urlParams.get('code');

        if (!verificationCode) {
            verificationStatus.textContent = 'Verification code not found.';
            return;
        }

        const mutation = `
        mutation VerifyEamail($input: VerifyEmail!){
            users{
            verifyEmail(input: $input) 
        } 
        }
        `;
        

        
        const variables = {
            input: {
                code: verificationCode,
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
                    verificationStatus.textContent = 'Verification failed. Please try again.';
                } else if (data.data && data.data.users && data.data.users.verifyEmail) {
                    // direct the user to a login page 
                    window.location.href = 'login.html';
                } else {
                    verificationStatus.textContent = 'Verification failed. Please check your email for the verification link.';
                }
            })
            
            .catch(error => {
                console.error('Network error:', error);
                verificationStatus.textContent = 'Network error. Please try again.';
            });
    });
});
