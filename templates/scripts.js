document.addEventListener('DOMContentLoaded', function () {
    const uploadForm = document.querySelector('#upload-section form');
    const fileInput = document.querySelector('#upload-section input[type="file"]');
    const mediaGrid = document.querySelector('#media-section .media-grid');

    uploadForm.addEventListener('submit', function (event) {
        event.preventDefault(); 

        const formData = new FormData();
        formData.append('operations', JSON.stringify({
            query: `mutation UploadMedia($input: UploadUserInput!) {
                images{
                    upload(input: $input)
                }   
            }`,
            variables: { file: null }
        }));
        formData.append('map', JSON.stringify({
            '0': ['variables.file']
        }));
        formData.append('0', fileInput.files[0]); 
        fetch('http://localhost:8000', {
            method: 'POST',
            body: formData,
        })
            .then(response => response.json())
            .then(data => {
                console.log('Success:', data);
                const imageUrl = data.data.upload; 
                uploadedImage.src = imageUrl; 
            })
            .catch(error => {
                console.error('Error:', error);
            });
    });
});

