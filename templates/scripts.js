document.addEventListener('DOMContentLoaded', function () {
    const uploadForm = document.querySelector('#upload-section form');
    const fileInput = document.querySelector('#upload-section input[type="file"]');
    const mediaGrid = document.querySelector('#media-section .media-grid');

    uploadForm.addEventListener('submit', function (event) {
        event.preventDefault(); 

        const formData = new FormData();
        formData.append('operations', JSON.stringify({
            query: `mutation ($file: Upload!) { upload(file: $file) }`,
            variables: { file: null }
        }));
        formData.append('map', JSON.stringify({
            '0': ['variables.file']
        }));
        formData.append('0', fileInput.files[0]); 
        fetch('http://localhost:8000', {
            method: 'POST',
            body: formData,
            headers: {
                'Content-Type': 'multipart/form-data'
            }
        })
            .then(response => response.json())
            .then(data => {
                console.log('Success:', data);
                const imageUrl = data.data.upload; // Adjust this line based on your server's response structure
                uploadedImage.src = imageUrl; 
            })
            .catch(error => {
                console.error('Error:', error);
                // Handle the error appropriately
            });
    });
});

